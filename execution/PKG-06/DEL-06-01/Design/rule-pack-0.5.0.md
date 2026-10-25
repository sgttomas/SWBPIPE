# Rule-pack format 0.5.0: stress categories, load-case mappings and required load cases (W-17)

**Basis:** `origin/main` at `146abe1389` (2026-10-25), with a code survey at
that commit. Designed, with two Codex design reviews (§7), and implemented:
slice A (format, validation, evaluation, run path, fixtures) and slice B
(desktop, demo pack 0.3.0, the report's load case per outcome; §6a). Standard claim fence applies (claims taxonomy per DEC-081).

**Requirement.** PRD §17.2 lists what a rule pack shall support, including
"required load cases", "stress categories" and "load-case mappings". FR-RULE-001
(Must): "Users can define private required inputs, stress categories,
expressions, allowables, load mappings, and blocking conditions." The owner
placed all three in v0.1 under a new rule-pack format version (2026-10-24: "I
approve your recommended option A.").

**Boundary.** Rule packs are user-supplied content (`docs/PRODUCT_BOUNDARIES.md`;
PRD §17.1, §17.4). The format carries the user's own category names,
descriptions and mappings. The repository adds no code-standard text, table,
allowable or category definition. Public examples and fixtures are invented and
labelled as such (FR-RULE-007; DEL-06-01 REQ-06-01-007, REQ-06-01-011).

## 1. Where things stand

- **Format.** Packs declare `schema_version` `0.4.0` (top level and
  `metadata.schema_version`). The validator
  (`core/rules/rule_pack_document/src/lib.rs` `validate_rule_pack_document`)
  reads the top-level value but never checks it; an empty or absent value is
  only caught as lifecycle `MISSING_VERSION`. `metadata.schema_version` is not
  read. The JSON schema checks the pattern only, and only Python tests apply it.
- **Store.** The desktop keeps the raw document in SQLite `local_rule_packs`
  (store v10; `apps/desktop/src-tauri/src/lib.rs`), with no format column.
  List does not validate, open re-validates and attaches findings, and run
  refuses a pack with blocking findings (`checks: []`).
- **Solver inputs.** A `solver_result` input binds one row of the solved
  mechanics envelope by its `id` (`RequiredInput.solver_result_ref.result_id`),
  or by a caller-supplied `{input_id, result_id}` selector when the pack carries
  no reference (`apps/desktop/src-tauri/src/lib.rs` `run_rule_checks_core`). The
  run panel sends such selectors from a per-input dropdown
  (`RuleCheckRunPanel.tsx`, `solverResultBindings`).
- **Rows and their basis.** Each envelope row carries
  `basis_ref: {ref_type: load_case | combination, ref_id}`, `kind`,
  `entity_ref` and usually `metadata.location`. Row ids are not a stable function
  of the basis: the first load case's rows are unprefixed
  (`result:stress:pipe-P-100:end-i:axial-normal`), other load cases are prefixed
  `result:loadcase:<case>:`, and combinations `result:combination:<id>:`
  (observed in `validation/witness/generated/tp_runner_015_final_cli_solve.json`).
  Exact-route `_v2` ids embed the case instead.
- **What separates rows within one basis.** `kind`, `entity_ref` and
  `metadata.location` are not enough. On the exact route, nominal and reduced
  section twins differ only by the `section_basis=` token in `metadata.basis`
  (`core/product_physics/src/lib.rs` `section_basis_row`), and several kinds
  publish one row per `metadata.component` (`Fx`/`Fy`/`Fz`, inner and outer
  hoop, `sigma_plus`/`sigma_minus`). In
  `fixtures/results/pressure_connected_mechanics_dense.json`, 82 of 183
  (basis, kind, entity, location) groups hold more than one row.
- **What a basis publishes.** Load-case rows carry `basis_ref` `load_case`;
  combination rows `combination`, for the linear-state kinds only
  (`preview_physics.rs` `LINEAR_STATE_KINDS`), so a combination has no
  `open_formula_stress_summary` row. A withheld combination publishes no rows.
  Exact models carry no combinations. Model-input echo rows have no
  `basis_ref`.
- **Evaluation.** Each check is evaluated once against the one row its input
  names. Nothing says which load case or combination a check concerns, so a
  pack cannot ask for "this check on every case of this kind", and a missing
  load case is not reported as such.

## 2. Decisions

1. **One current format, `0.5.0`.** Top-level `schema_version` and
   `metadata.schema_version` must both equal `0.5.0`.
   - Any other value, including `0.4.0`, an absent member or a top-level value
     that differs from `metadata.schema_version`, is a blocking validation
     finding `RULE_PACK_SCHEMA_VERSION_UNSUPPORTED`, naming the version found
     and the version supported. No check of that pack runs.
   - Validation still computes the checksum and its other findings. The model's
     rule-pack reference resolution (`services/modelRulePackResolution.ts`, the
     handoff's `handoff_package.rs`) reads the checksum match status, so a
     refused pack must not also appear as a checksum mismatch.
   - There is no migration and no reader for older packs. A stored `0.4.0`
     pack stays listed, opens with the refusal shown, is refused at run, and
     can be deleted. The user re-authors it. The store schema is unchanged.
   - `grammar_version` is unchanged (`1.0.0`, DEC-022).
2. **The binding lives in the pack and names the model's own ids.** A pack is
   already specific to its model: its solver inputs name model rows and
   entities. The mapping from the pack's categories to the model's load cases
   and combinations is therefore authored in the pack, by the user, and bound
   by the pack checksum. No model-document member is added; model document
   0.7.0 is unchanged.
3. **New top-level members** (all required, each at least one item):
   - **`required_load_cases`**: each item
     `{required_load_case_id, name, description?, basis_ref: {ref_type:
     load_case | combination, ref_id}, provenance}`. `ref_id` is the model's
     id for that load case or combination. Item ids are unique; two items may
     not name the same `basis_ref`.
   - **`stress_categories`**: each item `{category_id, name, description?,
     provenance}`. The name and description are the user's text. The format
     gives categories no built-in meaning and no fixed list.
   - **`load_case_mappings`**: each item `{mapping_id, category_ref,
     required_load_case_refs: [...]}` with at least one reference. Every
     category has exactly one mapping; every required load case appears in at
     least one mapping. A load case may feed more than one category.
4. **Checks name their category.** `CheckDefinition` gains a required
   `category_ref`. A check is evaluated once for each required load case mapped
   to its category (its **bases**), in the mapping's order.
5. **Solver inputs select a row within a basis.** In 0.5.0
   `solver_result_ref` is `{result_kind, entity_ref, location?, component?,
   section_basis?, tie_rod?}` and `result_id` is removed.
   - For each basis, the input binds the one envelope row whose `basis_ref`
     equals the basis, whose `kind` and `entity_ref` equal the selector's, and
     whose `metadata.location`, `metadata.component` and section basis equal
     the selector's optional members. An absent optional member matches only a
     row without that member. The section basis is the `section_basis=` token
     of `metadata.basis` (`nominal_section` or `reduced_section`); the runner
     reads it with one parser shared with the producer's convention, and a
     `metadata.basis` with a malformed token matches nothing.
   - `tie_rod` names one rod of a connector's `tie_rods` record. A
     `connector_tie_rod_force_v1` row carries its rod only in its id, so the
     match follows the result reader's binding of a rod row id to its rod
     (`core/reporting/result_export/src/connector_evidence.rs`
     `tie_rod_row_tail`, `is_tie_rod_row_id`; as built, §6a). The selector's
     `tie_rod` must equal that rod's id exactly, so an alias that encodes to
     the same row id (`rod-yp` for `rod:yp`) binds nothing. Two
     rods of one connector otherwise publish rows that the other members
     cannot separate (`fixtures/results/pressure_v3_connector_reader_corpus.json`,
     `load:U5A`, `component:C-J`, rods `yp` and `ym`).
   - **Eligibility gates are unchanged.** Every matched row passes the same
     gates a bound row passes today, before its value is used: the run's
     mechanics qualification (`qualify_rule_mechanics_with_context`) and the
     per-row refusal (`semantic_contract::rule_binding_refusal`, which refuses
     retained rows classified `AbsoluteVerified` or `NotCovered`). A refusal is
     recorded for that input on that basis, with its reason, and that basis is
     incomplete.
   - Rows without `basis_ref` are never selected.
   - A regression test asserts that, in every committed result and witness
     envelope (`fixtures/results/`, `validation/witness/`), every row that has
     a `basis_ref` is selected by exactly one selector within its basis,
     including the connector reader corpus. The review of this design found
     one collision group in 22,630 basis-bearing rows (the tie rods above) and
     none once `tie_rod` is a member.
   - No row: the input is unsupplied for that basis
     (`RULE_SOLVER_RESULT_ROW_MISSING`). More than one row:
     `RULE_SOLVER_RESULT_ROW_AMBIGUOUS`, also unsupplied. Neither is a pass.
   - A `solver_result` input without a `solver_result_ref` is a blocking
     validation finding in 0.5.0 (`RULE_SOLVER_RESULT_REF_MISSING`). The
     caller-supplied `{input_id, result_id}` selectors are retired: the run
     command refuses them by name for a 0.5.0 pack
     (`RULE_CALLER_SOLVER_BINDING_RETIRED`). One binding route remains.
   - User-supplied values, value slots and library values are the same for
     every basis.
6. **Required load cases block when missing.** Before evaluating, the run checks
   each required load case against the current model and the envelope.
   - Absent from the model's `load_cases` or `combinations`:
     `RULE_REQUIRED_LOAD_CASE_MISSING`.
   - Present in the model but with no rows in the envelope (for example a
     withheld combination): `RULE_REQUIRED_LOAD_CASE_UNSOLVED`.
   - Solved, but with no row of the selected kind (for example a formula
     stress summary on a combination): the input's
     `RULE_SOLVER_RESULT_ROW_MISSING` for that basis.
   - The run command passes the current model to the core for these checks;
     today only the qualification step sees it.
   - Either makes every check whose category maps that load case
     `RULE_INPUTS_INCOMPLETE` for that basis, with the finding naming it. A
     check is never evaluated on a basis it cannot bind.
7. **Outcomes: one per check and basis.** The run result holds one
   `CheckOutcome` for each check on each of its bases, in check order and then
   mapping order.
   - Each outcome keeps today's members (`check_id`, status, computed and limit
     values, relation, bound inputs, findings, diagnostic codes), all for that
     basis alone, and adds `category_ref` and `basis_ref`. Nothing is merged
     across bases, so no value, input or finding is shown for a basis it did
     not come from.
   - The pack aggregate is unchanged in rule (FR-RULE-003; DEL-06-03 Design
     `pack-aggregate-incomplete.md`): it ranks incomplete above failed above
     checked over all outcomes. Its counts now count outcomes, so a check that
     fails on one load case and is incomplete on another is counted once
     failed and once blocked, and its failure stays visible under the
     INCOMPLETE headline. The headline text is unchanged; "check(s)" there
     counts checks on their load cases.
   - A check with no solver inputs is evaluated on each of its bases like any
     other; its outcomes differ only by basis.
   - `schemas/rule_check_run_result.schema.json` gains `category_ref` and
     `basis_ref` on `CheckOutcome`.
   - **Outcome identity** is the serialized tuple (`check_id`,
     `basis_ref.ref_type`, `basis_ref.ref_id`). Outcome keys, test ids and
     lookups use that identity (today the run panel keys and test-ids outcomes
     by `check_id`, `RuleCheckRunPanel.tsx`, and a runner test helper returns
     the first matching check id, `tests/interval_bounds_run.rs`).
   - **Count scope is shown.** Beside the unchanged headline, the run panel
     says: "Counts are per check and load case or combination; one check can
     contribute more than once." DEL-06-03 Design `pack-aggregate-incomplete.md`
     says the same of its counts. The report's rule-check section carries the
     same note (§6).
   - **Consumers keyed by `check_id`** now see it repeated once per basis. The
     run panel shows each outcome with its category and load case. The report
     is §6.
8. **Units are unchanged.** Each basis converts its bound values to the
   declared units exactly as a single evaluation does today (B2/B3
   normalization in `rule_check_runner`).
   - **Interval mode.** A verified bound (`SolverResultBound`) names its input
     and its basis and applies to that basis only; a bound without a basis is
     refused. Only tests use bounds today.
   - **Cost.** Rows are indexed once per run by basis, kind and entity. The
     number of evaluations is the sum over checks of their mapped bases, which
     the pack's own size bounds; v0.1 sets no separate work limit.
9. **Static validation of the new members** (blocking findings, each named):
   - unknown references (a mapping's category or load case, a check's
     category): `RULE_PACK_REFERENCE_UNRESOLVED`;
   - duplicate ids, a duplicate `basis_ref`, or the same required load case
     twice in one mapping: `RULE_PACK_DUPLICATE_ID`;
   - a category with no mapping or more than one, or a required load case in no
     mapping: `RULE_PACK_MAPPING_INCOMPLETE`;
   - every required load case must be reachable from at least one check
     through that check's category mapping; otherwise validation reports
     `RULE_PACK_MAPPING_INCOMPLETE`. Without this, a required load case mapped
     only to a category with no checks would produce no outcome, and a missing
     load case could escape the aggregate.
   - a required load case or stress category without `name`, with a
     non-string `description`, or without `provenance`:
     `RULE_PACK_RECORD_INCOMPLETE`; a provenance record missing any of its
     members or holding a value outside its allowed set:
     `RULE_PACK_PROVENANCE_INCOMPLETE`.
   Whether a named load case exists in a model is a run-time question (§2.6),
   not a static one, because a pack is validated without a model.
10. **Checksum.** The new members sit inside the JCS-canonical payload hashed by
    `rule_pack_checksum`, so the mapping is bound to the pack identity that the
    model's `rule_pack_refs`, the analysis run and the report cite.

## 3. Rule Pack Manager authoring and validation

In `features/rule-packs/` (DEL-07-03; `RulePackManagerPanel.tsx`,
`DeclarationsEditor.tsx`, `CheckDefinitionsEditor.tsx`):
- **New pack** writes `0.5.0` and seeds one category, one mapping and one
  required load case with empty text for the user to fill. Nothing is
  pre-filled with a category name of any standard.
- **Required load cases** editor: name, description, and a basis chosen from
  the current model's load cases and combinations (by id and label), or typed
  when no model is open.
- **Stress categories** editor: name and description.
- **Load-case mappings**: per category, the required load cases it uses.
- **Checks**: a category selector.
- **Solver inputs**: the row selector's members, typed. When the session has a
  current solved result, the editor also offers its rows' selectors (the panel
  gains a prop for the current result; it receives only the model today).
- **Run panel** (`RuleCheckRunPanel.tsx`): the per-input selector dropdown and
  `solverResultBindings` are removed; it shows each check's category and
  bases. Its input preview and binding pre-check cover only the bases of the
  checks that use each input (`ruleCheckService.ts` `solverInputBases`), the
  runner's own scope.
- **Validation** shows the §2.9 findings and the version refusal by code. A
  stored `0.4.0` pack shows the refusal and offers delete only.

## 4. Fixtures, examples and their writers

- `examples/rule_packs/invented_demo.yaml` and
  `fixtures/product_preview/invented_demo_rule_pack.json` (byte-identical) move
  to `0.5.0`, with an invented category ("Invented category A") and required
  load cases naming the bundled demo model's `load:L-100` and `load:L-200`.
  Their notices stay invented and non-engineering.
  - The demo pack demonstrates the refusal path, as today: its solver input's
    selector names the invented kind `invented_demo_quantity`, which no
    producer publishes, and its limit slot stays unsupplied. On every model it
    is `RULE_INPUTS_INCOMPLETE` naming `demo_actual_quantity` and
    `demo_limit_quantity` as missing, as the native smoke requires.
  - `tests/test_invented_example_models.py` compares the YAML's raw-byte
    SHA-256 with `rule_pack_refs[].checksum` in `examples/models/invented/*.json`;
    those references are re-stamped by their writer.
- Every fixture, test pack and builder that writes a rule pack moves to `0.5.0`
  through its writer (to confirm by the survey: Rust test builders, Python
  schema fixtures, desktop test builders, e2e).
- `0.4.0` packs remain only as refusal fixtures.

## 5. Tests

- **Format.** A `0.5.0` pack validates; `0.4.0`, an absent version and a
  mismatched `metadata.schema_version` are each refused by
  `RULE_PACK_SCHEMA_VERSION_UNSUPPORTED`. Each §2.9 finding is reached.
- **Round trip.** A `0.5.0` pack saved to the local store and read back is
  byte-identical in canonical form, and its checksum verifies. A stored `0.4.0`
  pack is refused on read and on run, by name, and can be deleted.
- **Evaluation (numerical meaning; independent scrutiny).** On an invented
  two-case model with a combination:
  - a check mapped to two load cases evaluates on each, with each basis bound
    to its own row (values differ by case), and the check fails if either
    basis fails;
  - a check mapped to a combination binds the combination's row;
  - one basis missing (`RULE_REQUIRED_LOAD_CASE_MISSING`) or unsolved makes
    the check incomplete while its other bases are still reported;
  - an ambiguous selector and a missing row are incomplete, never pass;
  - the pack aggregate follows FR-RULE-003 over the outcomes, and a check that
    fails on one basis and is incomplete on another shows both in the panel
    and the report;
  - a subtraction and a range-envelope combination as bases;
  - a check with no solver inputs, a load case shared by two categories, and a
    duplicate reference within one mapping (refused statically);
  - a bound applied to one basis does not affect another;
  - a retained row that `rule_binding_refusal` refuses is refused on its basis
    and never bound;
  - two tie rods of one connector are selected separately;
  - a required load case mapped only to a category with no checks is refused
    statically (`RULE_PACK_MAPPING_INCOMPLETE`);
  - outcome identity: repeated check ids are told apart by basis, stable
    across a rerun and a reordering of the envelope's rows; one check failing
    on three bases shows three failed with the count-scope note;
  - a single-basis pack gives the same computed value and status as the same
    check bound by row id before the change.
- **Desktop.** Manager editors and validation; the run panel shows each check's
  bases; the report's per-check projection is unchanged.
- **Native smoke.** The demo-pack steps on the demo and exact models still
  refuse as `RULE_INPUTS_INCOMPLETE` naming both inputs missing; `run.mjs` and
  `expected.json` are unchanged.
- **Schemas.** `rule_check_run_result.schema.json` gains a `BasisRef`
  definition; `tests/test_operation_result_schemas.py`, which pins the
  definition set, lists it.
- **e2e.** `r2-smoke.spec.ts` asserts the run panel's caller selector; it moves
  to the authored selector.

## 6. Effects outside DEL-06

- **Reports.** The report's projection (`renderableReportInput.ts`
  `userRuleChecks`) carries each outcome's `basis_ref`. The renderer's
  `UserRuleCheckOutcome` (`core/reporting/report_renderer`) gains an optional
  `basis_ref`. The rule-check section shows it in a "Load case or combination"
  column, with "load case not recorded" when absent, and shows the count-scope
  note after the headline. An empty `basis_ref.ref_id` is refused
  (`USER-RULE-CHECK-BASIS-REF-INVALID`).
  - **Package version.** The member is added inside the 2.0.0 report package.
    The owner ruled W-18b O-A as V1 (2026-10-25: "I approve your recommended
    options for the three W-18b decisions."). Under that reading the report
    package stays at 2.0.0, and DEC-028's "one version" admits a member added
    at a head, which the reader at that same head accepts.
  - The `report_package` reader needed no source change. A regression test
    reads a 2.0.0 package with and without `basis_ref`. The package's
    rule-check binding (aggregate and pack identity) is unchanged.
  - The reports lane's W-11 design (`DEL-08-01 Design
    exact-route-report-sections.md`) records this shape. This change edits
    only the rule-check hunks of the shared files.
- **Model document, analysis runs, handoff:** unchanged. They cite packs by id,
  version and checksum.
- **Solver:** unchanged. No product_physics file is touched.

## 6a. As built in slice A

- **Shared format module.** `core/rules/rule_pack_document/src/format.rs` holds
  the version, finding codes, `BasisRef`, the selector parser and each check's
  basis plan, used by both the validator and the runner.
- **References.** `category_ref` and `required_load_case_refs` are `Reference`
  objects with `ref_type` `stress_category` and `required_load_case`.
- **Row selection** is `apps/desktop/src-tauri/src/rule_solver_selection.rs`.
  - **Tie rods.** `connector_evidence.rs` is priced by the retained-pricing
    rules, so it is not changed. The module restates the rod-row id convention,
    citing `connector_evidence.rs` and `core/product_physics/src/joint.rs`.
    `row_tie_rod` reads the rod ids from the envelope's connector records, the
    records the reader binds to. A row has a rod only when its connector has
    exactly one record and exactly one of that record's rods encodes to the
    row id; the selector's `tie_rod` must then equal that id exactly. The TS
    mirror is `ruleCheckService.ts` `rowTieRod`. A test cross-checks both the
    matcher and `row_tie_rod` against the reader on its corpus, and a forged
    rod fails both. The convention covers load-case rows only, so a tie-rod row on
    a combination would be row-missing; no committed envelope has one.
- **Gates.** Mechanics qualification runs once per run, as before;
  `rule_binding_refusal` runs on every matched row.
- **Refusal order.** Any non-null caller `solverResultBindings` is refused
  after validation, so a `0.4.0` pack is refused by its version first. A bound
  without a basis refuses its input on every basis.
- **`basis_ref` may be null** only on an outcome for a check with no bases,
  which validation already blocks; such an outcome is incomplete.
- **Single-basis equivalence (pinned).** `pressure_connected_mechanics_dense.json`
  row `…:end_i:axial_membrane_stress:reduced_section` against an invented
  7.3 MPa limit gives 0.62266500622665 (`0x3fe3ecdf29c36d59`),
  `USER_RULE_CHECKED`, the same as the runner before the change at
  `38e5b73d5b`.
- **Selector uniqueness** covers 37 files, 55 envelopes and 20,856
  basis-bearing rows, the connector corpus included.

## 6b. As built in slice B

- **Manager.** `App.tsx` passes the current solved result to the Rule Pack
  Manager, which offers its rows' selectors. New packs are drafted at `0.5.0`
  (`rulePackService.ts`). The §2.9 findings and the version refusal are shown
  by code, and a stored `0.4.0` pack offers delete
  (`RulePackManagerPanel.format.test.tsx`).
- **Run panel.**
  - The caller selector is gone, and outcomes are keyed by (check, basis).
  - A run refused by validation shows its finding codes.
  - The policy line reads
    `solver_selectors=N; solver_selectors_missing=M; required_load_cases=K`.
  - In the TS outcome type, `category_ref` and `basis_ref` are optional; the
    runner always sends both.
- **Demo pack** `0.2.0` → `0.3.0`, checksum
  `c96d28d94a1d4eee99ae2f501c3d4d5748fb81bfa7feb034cc8d96d60651e68f`.
  - Re-derived by their writers: the desktop and applier emission goldens, the
    state-comparison golden, the handoff witness and the parity cases.
  - Restamped with the function their tests check, because they have no
    writer:
    - `invented_every_slot_model` and `invented_annotated_model`;
    - the rule-pack reference resolution vectors (the "version differs"
      vector now edits to `0.4.0`);
    - `examples/models/invented/fake_rule_pack_toy_model.json`.
- **Replaced test assertions.** Each was replaced by the equivalent check of
  the new behaviour:
  - In `e2e/r2-smoke.spec.ts`, `solver_result_refs=0` became
    `solver_selectors_missing=0`.
  - The caller dropdown checks became the authored selector text, plus a check
    that no caller control exists.
  - The display-units picker checks went with the picker; the selector
    preview's unit conversion is still checked.
  - The retained-precision notice test excludes only rows without a
    `basis_ref`, which cannot be bound. It asserts that every other row has a
    selector that selects it alone, and checks the notice over all of them.
    On its ten forms (125 rows each) this is all 125 rows, as on `main` before
    0.5.0.

- **Seam scanner witness.** `core/reporting/result_export/tests/retained_precision_carriers.rs`
  `u6_semantic_seams_have_only_governed_product_callers` checks its scanner
  with four product functions that sit after a file's first `#[cfg(test)]`.
  One of them, `lib.rs` `solver_result_row_value`, was removed when row binding
  moved to `rule_solver_selection.rs`. Its replacement is `lib.rs`
  `qualify_rule_mechanics_with_context`, the rule-binding gate, which sits in
  the same position, so the check keeps four witnesses. The edit is in
  `tests/`, which retained pricing does not scan. The file's runtime-read
  declaration in `tools/hosted-ci-routing.json` is restamped; its inputs are
  unchanged.

## 7. Review findings resolved

The Codex design review of `b1d55e38ea` found these; the design above resolves
them:
1. **Tie rods could not be selected separately (blocking).** `tie_rod` joins
   the selector, through the reader's existing rod-row binding; the uniqueness
   regression covers the connector corpus (§2.5).
2. **A rolled-up check status hid a known failure (blocking).** One outcome per
   check per basis replaces the roll-up, so the existing aggregate counts the
   failure and the report shows it with its load case (§2.7, §6).
3. **Representative values and merged inputs were underspecified.** With
   per-basis outcomes nothing is merged (§2.7).
4. **Binding refusals had to be kept explicitly.** Every matched row passes the
   existing qualification and `rule_binding_refusal` gates, per input and basis
   (§2.5), with tests (§5).

Round 2 (`8d5b6495da`) confirmed those and found:
5. **A required load case could escape the aggregate (blocking).** Every
   required load case must now be reachable from a check (§2.9).
6. **Outcome identity** is the (check, basis) tuple everywhere (§2.7).
7. **Count scope** is stated beside the headline (§2.7).

Both rounds reviewed the ScopeOfWork wording in §8 and approved it as written.

The implementation review of `c5b6f6a17d` found:
8. **A tie-rod alias bound the real rod (blocking).** Exact rod-id equality
   against the connector record (§2.5, §6a).
9. **The new records' required members were not validated.** Two blocking
   codes (§2.9).
10. **The run panel previewed an input on load cases it does not use.** The
    preview follows the runner's basis plan (§3).
11. **The retained-notice test's scope was filtered through the selector.** It
    now excludes only unbindable rows and asserts selector uniqueness for the
    rest (§6b).

## 8. ScopeOfWork wording (applied)

Applied to DEL-06-01 `ScopeOfWork.md` CLM-011 under the owner's wording grant
(LOOP_INIT, 2026-10-23). Codex reviewed the exact text in both design rounds
and approved it; the coordinator confirmed it (2026-10-25).
- REQ-06-01-002, replaced. Old: "The schema shall include rule-pack identity,
  schema version, rule-pack version, lifecycle/status, and source-note
  metadata." New: the same sentence, then "One schema version is current; a
  document declaring any other version is refused by name, without
  migration."
- REQ-06-01-013, added: "The schema shall support user-defined stress
  categories, required load cases that reference model load cases or
  combinations, and mappings that assign required load cases to each category.
  Each check names one category and has one outcome for each load case or
  combination mapped to it. A mapped load case or combination that is absent
  from the model, has no result rows, or lacks a row the check needs makes that
  outcome incomplete without evaluating the check on it; the check's other
  outcomes are reported independently."
