# Unit check at rule-pack validation (W-17)

**Basis:** `origin/main` at `356a9be374` (2026-10-23). §2.1–2.6, §3 and §4 are
implemented in W-17 (`check_expression_types` in `expression_evaluator`,
`check_rule_pack_units` in `rule_pack_document`); §2.8 lists the as-built
differences. §2.7 is not implemented. Standard claim fence applies (F-PIP-2; claims taxonomy per
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
   - **Shared rules.** It applies the evaluator's own rules: the same helpers
     for dimension products and quotients, composed units and unit equality. It
     produces the evaluator's `FindingCode`s and subject ids.
   - **Type states.** Each variable in the environment is in one of three
     states:
     - `Typed(dimension, unit_ref)`, with the metadata flags the runner gives
       every bound quantity (`unit_required` and `dimension_check_required` both
       true; `Quantity::new`, `expression_evaluator/src/lib.rs:106-124`, called
       at `rule_check_runner/src/lib.rs:862`);
     - `Unknown`: declared, but with `TBD` or empty metadata;
     - `Undeclared`.

     An `Unknown` operand gives an unknown result with no finding, and no
     ancestor reports a finding against it. A node whose operands are all
     typed is checked by the evaluator's rule.
   - **"Static"** means independent of runtime bindings: values, binding
     presence, statuses. Literals may be inspected: `table_malformed` reads the
     pack's literal table rows (`expression_evaluator/src/lib.rs:866-880`), and
     those rows are pack content.
   - **Traversal differs from the evaluator's.** The evaluator stops a node at
     the first failing child (`?` at `:545-546`) and tests a zero divisor before
     dimension compatibility (`:1353-1360`). The static check has no values, so
     it visits every child and never reports `division_by_zero`. A type error
     that the evaluator hides behind a value failure is still reported
     statically.
   - It changes no grammar, so no version bump or corpus extension is needed.
2. **What is known statically.** S is every finding the evaluator can derive
   from the expression, its literals and the variables' types:
   - `type_mismatch`, `dimension_mismatch`, `unit_mismatch`;
   - `unsupported_expression_form`: an empty min or max, an unrepresentable
     product, an ambiguous or unrepresentable quotient, a refusal marker;
   - `unsafe_construct`;
   - `invalid_reference` (an empty variable id);
   - `missing_variable`;
   - `unit_metadata_missing`;
   - `table_malformed`, since tables are wholly pack literals.

   **The environment, per check.** In pack validation, each check gets its own
   environment.
   - A variable is `Typed` or `Unknown` when it is listed in both the formula's
     `input_refs` and the check's `required_input_refs`, and declared in
     `required_inputs`. Its declaration's `quantity_intent` sets the type.
     The runner binds exactly the check's required inputs
     (`rule_check_runner/src/lib.rs:435-470`), so this matches what a run can
     bind.
   - Any other variable is `Undeclared`.

   An authored refusal marker already blocks through `REFUSAL_MARKER_AUTHORED`,
   so the check reads `not_checkable` and the finding is not repeated.

   **Check-level findings** that validation adds:
   - `RULE_PACK_UNDECLARED_VARIABLE`: an `Undeclared` variable.
   - `RULE_PACK_UNIT_DECLARATION_INCOMPLETE`: an `Unknown` input, or a declared
     slot with `TBD` or empty metadata. The check is then partly untypable.
   - **Limits, which are checked only for quantity formulas.** A Boolean
     formula is a predicate and never reads a limit
     (`rule_check_runner/src/lib.rs:955-961`). `resolve_limit` takes the first
     referenced slot that has a supplied binding, using the slot's declaration
     when there is one and the binding's own unit otherwise (`:1154-1195`).
     Which slot is used therefore depends on the run. So, for each slot in
     `value_slot_refs`:
     - **A declared slot whose `(dimension, unit_ref)` differs from the
       formula's type** gives `RULE_PACK_LIMIT_CANDIDATE_MISMATCH`, naming the
       slot. The check fails if that slot is the one used.
     - **A slot that is not declared** gives `RULE_PACK_LIMIT_SLOT_UNDECLARED`
       (a warning). Its type comes from the binding at run time, so nothing is
       claimed about it.
     - **If every referenced slot is declared and incompatible,** the finding
       is `RULE_PACK_LIMIT_UNIT_MISMATCH`: the verdict comparison fails on
       every run.
     - **No referenced slot at all:** nothing is reported here; the runner
       reports the missing limit.

     Runtime selection is not changed.
3. **What is known only at run time:**
   - the unit of each solver result, supplied value or library value, and
     whether it converts to the declared unit (`:1197-1263`);
   - missing or duplicate bindings;
   - value-dependent findings: out of range, key not found, division by zero,
     non-finite values and overflow;
   - interval indeterminacy;
   - which solver field binds when the pack authors no `solver_result_ref`, and
     the unit of the bound solver value (`src-tauri/src/lib.rs:2932-2945`);
   - which value slot supplies the limit (above).
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
   - **Severity follows §2.2.** Expression findings in S and
     `RULE_PACK_LIMIT_UNIT_MISMATCH` (unconditional) are blocking for their
     check: that check fails on every run.
     - `RULE_PACK_LIMIT_CANDIDATE_MISMATCH` is blocking only if its slot is the
       one used at run time. It is reported as conditional, not blocking.
     - `RULE_PACK_LIMIT_SLOT_UNDECLARED`, `RULE_PACK_UNIT_NOT_IN_CATALOG` and
       `RULE_PACK_OUTPUT_DIMENSION_DIFFERS` are warnings.
   - **None of these enter `has_blocking_findings`.** The run therefore still
     evaluates the other checks, and an inconsistent check still blocks at run
     time with the same code, so run outcomes, the corpus and the runner tests
     are unchanged.
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
   - **The blocked-run response is projected.** A run whose pack fails
     validation returns `rule_pack_validation_value(...)` whole, inside the run
     response (`src-tauri/src/lib.rs:2913-2929`). That embedded validation
     omits `unit_check`. A projection helper removes the member there, and only
     there; `validate_rule_pack`, save and open keep it.
   - **Ids and unit tokens may themselves be private.** They are shown only in
     the local panel, and the projection keeps them out of run responses.
7. **A related runtime defect, fixed in a separate slice.**
   - **The defect.** `extract_library_slot_value` passes a material value's
     `unit_ref.ref_id`, a catalog id such as `unit:pascal`, on as a unit
     symbol (`src-tauri/src/lib.rs:3155-3166`). The runner then calls
     `unit_by_symbol` on it (`rule_check_runner/src/lib.rs:1197-1240`), so a
     `unit:pascal` value bound to a `Pa` declaration fails with
     `UnitMismatch`.
   - **Why tests miss it.** The tests use matching `demo_unit` strings on both
     sides (`src-tauri/src/lib.rs:7990-7998`).
   - **The fix.** A shared catalog-id-to-symbol resolver in `src-tauri`, built
     from `open_pipe_stress_units::catalog()` as `get_unit_catalog` is
     (`src-tauri/src/lib.rs:419-449`). It does not edit `core/units`.
     - An id in the `unit:` namespace that the catalog holds becomes its
       symbol.
     - A `unit:` id the catalog lacks is refused. The binding is omitted, and
       the input blocks as unsupplied.
     - A string outside the `unit:` namespace passes through unchanged as
       today, so identical-string bindings such as `demo_unit` keep working.
       That is consistent with the warning-only policy of §2.4.
     - That pass-through applies to rule binding only. Library apply (DEL-03-01
       Design `apply-library-records.md` §2.4) uses the same resolver with a
       stricter caller check, which refuses anything that is not a recognised
       catalog id.
   - **This is an intended change in runtime behaviour.** It gets its own
     regression check: a `unit:pascal` value bound to `Pa` and to `MPa`
     declarations; an unknown `unit:` id refused; `demo_unit` unchanged.
   - **It lands as its own slice,** separate from the static check. It changes
     run outcomes, which the static check does not. W-17 item 2's library apply
     uses the same resolver.
   - **Status: not implemented.** The runtime defect above remains.
   - **A related case, found at implementation.** A dimensionless limit slot
     declared with the catalog symbol `1` never matches the evaluator's token
     `ratio` that a ratio formula produces.
     - The static check reports it as `RULE_PACK_LIMIT_UNIT_MISMATCH`, which
       matches run time: the same check blocks there with `UnitMismatch`.
     - The resolver slice decides whether `1` and `ratio` resolve to one unit.
       Until then, authors of dimensionless limits see the finding at
       validation and the block at run time.
8. **As built (differences from 1–6).**
   - **Undeclared variables.** An undeclared variable is reported only as
     `RULE_PACK_UNDECLARED_VARIABLE`; the evaluator's `MissingVariable` is not
     also shown.
   - **Codes.** Static expression findings use the runner's code spelling
     (`UnitMismatch`), so static and run-time codes match.
   - **Severity.** `RULE_PACK_UNIT_DECLARATION_INCOMPLETE` is a warning.
   - **Not checkable.** A check that is `not_checkable` has no findings and no
     reason field.
   - **The invented demo pack** gets three catalog warnings: two for
     `demo_unit`, and one for its slot's `ratio`, which is the evaluator's ratio
     token but not a catalog symbol.

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
  test). The adapter mirrors the evaluator's order: `build_binding_map`
  processes every binding first (`expression_evaluator/src/lib.rs:430-469`).
  - **The environment is built from all of a case's `bindings`, not from
    `required_variable_ids`:**
    - a binding with a quantity gives `Typed`;
    - a binding whose `quantity` is null gives `Unknown`, not missing (case
      62);
    - a binding whose quantity lacks the required metadata gives
      `unit_metadata_missing` and is then treated as unbound. Case 65 needs
      this: its `required_variable_ids` is empty and its finding comes from the
      binding alone
      (`fixtures/rule_expressions/conformance_corpus/case_65_block_unit_metadata_missing.json`);
    - an AST variable with no usable binding gives `Undeclared`, reported as
      `missing_variable`.
  - **`required_variable_ids` is used separately,** only for checks that need
    it. It does not build the type environment.

  Two properties are checked:
  - **Soundness, for every case:** each expected finding in S appears among the
    static findings, with the same code and subject.
  - **Equality, only on comparable cases** (cases whose expected findings all
    lie in S, or which have none): the static findings equal the expected ones.

  Cases with a value- or binding-dependent finding are not compared for
  equality: 44-48, 58, 62, 63 and 66, and the grammar-version cases 30-32. The
  test does not filter dynamic findings by code to force a match.
- **Mixed cases** (new test inputs in the test file; the corpus files are
  unchanged):
  - case 57's operands with a zero divisor (`stress` 12.0 divided by `time`
    0.0). The evaluator reports `division_by_zero` (the zero test comes first,
    `:1353-1360`). The static check reports `unsupported_expression_form`,
    subject `divide`, the same structural finding case 57 expects for an
    unrepresentable quotient (`:1397`). Division never yields
    `dimension_mismatch`;
  - a left operand out of table range with a unit mismatch on the right: the
    evaluator reports only the range finding, and the static check reports the
    mismatch.
- **Validation tests** (`rule_pack_document`):
  - an undeclared variable;
  - limits:
    - a Boolean predicate with an incompatible slot: no finding;
    - a single incompatible declared slot: `RULE_PACK_LIMIT_UNIT_MISMATCH`;
    - one compatible and one incompatible slot: a candidate finding only;
    - an undeclared slot: a warning only;
  - an incomplete declaration;
  - a non-catalog unit as a warning, with the demo pack showing only that
    warning;
  - an `output_dimension` difference.
- **Unchanged outputs.** `corpus_parity.rs` and `invented_demo_document.rs` are
  extended only by the additive member. `has_blocking_findings` is unchanged for
  every existing fixture.
- **Privacy tests:**
  - a pack with a sentinel magnitude, table row and payload summary: the
    serialized `unit_check` contains none of them;
  - adversarial private strings used as ids and unit tokens: they appear in
    `validate_rule_pack`'s `unit_check`;
  - the blocked-run branch of `run_rule_checks_core`
    (`src-tauri/src/lib.rs:2913-2929`): its response carries no `unit_check`
    and none of those strings.
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
| `core/rules/rule_pack_document/Cargo.lock`, `core/rules/rule_check_runner/Cargo.lock`, `apps/desktop/src-tauri/Cargo.lock` | the dependency of `rule_pack_document` on `open_pipe_stress_units`. These are the only lockfiles that contain `rule_pack_document`. | none expected | no |
| `core/product_physics/Cargo.lock` (bound by the registered profile, `retained_memory.rs:1204`) | untouched: it does not contain the rules crates | S1b's D3 chain | no |
| `apps/desktop/src/services/rulePackService.ts` | type | none | no |
| `apps/desktop/src/features/rule-packs/RulePackManagerPanel.tsx` | findings group | S2 adds "Use with this model" and "Remove from model" | same file, different region; no |
| `apps/desktop/src-tauri/src/lib.rs` | this slice: the blocked-run projection (`:2913-2929`). Separate slice (§2.7): the shared resolver and `extract_library_slot_value` (`:3155-3166`) | S1b: self-test model (`:4482`), open-path validator | same file, different code; no |
| D3-pinned files | none; `core/units/src` is only depended on, not edited | S1b's D3 chain | no |

**Dependencies.** This item touches no S1b or S2 file except the panel, and no
model-document member, so it does not depend on S1b. The §2.7 runtime fix is a
separate slice with no dependency on this one.

## 6. Review findings resolved

The Codex review of `1f1b0d7fa9` found these, and the design above resolves
them:
1. **A limit mismatch is not always unconditional (P1).** Predicates skip
   limits; findings are candidate-specific or unconditional; undeclared slots
   get a warning; runtime selection is unchanged (§2.2).
2. **The static contract and parity oracle were underspecified (P2).**
   Resolved by defining "static", the three type states, traversal
   differences, the per-check environment, soundness plus equality on
   comparable cases, and the mixed tests (§2.1, §2.2, §4).
3. **Diagnostics would enter blocked-run responses (P2).** Resolved by the
   projection and its tests (§2.6, §4).
4. **The library unit-ref defect (P2, existing).** Recorded, with its fix in a
   separate slice (§2.7).
5. **Lockfiles.** Named exactly; the pinned `core/product_physics/Cargo.lock`
   is unaffected (§5).

Codex round 2 (on `63615afe87`):

6. **R2-5, wrong quotient expectation (P2).** The zero-divisor example now
   expects `unsupported_expression_form`, as an unrepresentable quotient
   gives.
7. **R2-5, corpus adapter built from the wrong list.** It is now built from
   all bindings, in the evaluator's order, with null quantities `Unknown` and
   `required_variable_ids` used separately.
8. **R2-5, blanket severity wording.** §2.5 now keeps §2.2's distinction
   between candidate-specific and unconditional findings.
9. **The shared unit resolver.** It now has a stricter caller check for library
   apply (§2.7).
