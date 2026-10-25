# Rule-pack format 0.5.0: stress categories, load-case mappings and required load cases (W-17)

**Basis:** `origin/main` at `146abe1389` (2026-10-25). Designed, not yet
implemented. Standard claim fence applies (claims taxonomy per DEC-081).

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
  does not check either value, so any version string is accepted.
- **Solver inputs.** A `solver_result` input binds one row of the solved
  mechanics envelope by its `id` (`RequiredInput.solver_result_ref.result_id`),
  or by a caller-supplied `{input_id, result_id}` selector when the pack carries
  no reference (`apps/desktop/src-tauri/src/lib.rs` `run_rule_checks_core`).
- **Rows and their basis.** Each envelope row carries
  `basis_ref: {ref_type: load_case | combination, ref_id}`, `kind`,
  `entity_ref` and usually `metadata.location`. Row ids are not a stable function
  of the basis: the first load case's rows are unprefixed
  (`result:stress:pipe-P-100:end-i:axial-normal`), other load cases are prefixed
  `result:loadcase:<case>:`, and combinations `result:combination:<id>:`
  (observed in `validation/witness/generated/tp_runner_015_final_cli_solve.json`).
- **Evaluation.** Each check is evaluated once against the one row its input
  names. Nothing says which load case or combination a check concerns, so a
  pack cannot ask for "this check on every case of this kind", and a missing
  load case is not reported as such.

## 2. Decisions

1. **One current format, `0.5.0`.** Top-level `schema_version` and
   `metadata.schema_version` must both equal `0.5.0`.
   - Any other value, including `0.4.0` and an absent member, is a blocking
     validation finding `RULE_PACK_SCHEMA_VERSION_UNSUPPORTED`, naming the
     version found and the version supported. Nothing else is evaluated for
     that pack.
   - There is no migration and no reader for older packs. A stored `0.4.0`
     pack in the local store is listed, refused by that code wherever it is
     validated or run, and can be deleted. The user re-authors it.
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
   `solver_result_ref` is `{result_kind, entity_ref, location?}` and
   `result_id` is removed.
   - For each basis, the input binds the one envelope row whose `basis_ref`
     equals the basis and whose `kind`, `entity_ref` and `metadata.location`
     equal the selector (`location` absent matches a row with no location).
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
   - Either makes every check whose category maps that load case
     `RULE_INPUTS_INCOMPLETE` for that basis, with the finding naming it. A
     check is never evaluated on a basis it cannot bind.
7. **Outcomes: one per check, with its bases.** The run result keeps one
   `CheckOutcome` per check, so the pack aggregate (FR-RULE-003; DEL-06-03
   Design `pack-aggregate-incomplete.md`) and the report's per-check projection
   (`renderableReportInput.ts` `userRuleChecks`) are unchanged in shape.
   - The outcome adds `category_ref` and `basis_outcomes`: for each basis, its
     `basis_ref`, status, computed and limit values, bound inputs and findings.
   - The check's status is the worst of its bases by the existing rank
     (incomplete above failed above checked). A check is checked only if every
     basis is checked.
   - The check's `computed_value` and `limit_value` are those of the first
     basis, in mapping order, that has the check's status.
   - The check's `bound_inputs` list each input once: supplied only if supplied
     in every basis; a missing input is listed once however many bases lack it.
   - `schemas/rule_check_run_result.schema.json` gains the two members.
8. **Units are unchanged.** Each basis converts its bound values to the
   declared units exactly as a single evaluation does today (B2/B3
   normalization in `rule_check_runner`).
9. **Static validation of the new members** (blocking findings, each named):
   - unknown references (a mapping's category or load case, a check's
     category): `RULE_PACK_REFERENCE_UNRESOLVED`;
   - duplicate ids or duplicate `basis_ref`: `RULE_PACK_DUPLICATE_ID`;
   - a category with no mapping or more than one, or a required load case in no
     mapping: `RULE_PACK_MAPPING_INCOMPLETE`.
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
- **Checks**: a category selector. Solver inputs get the row selector
  (kind, entity, location), offered from the current solved result's rows when
  one exists.
- **Validation** shows the §2.9 findings and the version refusal by code. A
  stored `0.4.0` pack shows the refusal and offers delete only.

## 4. Fixtures, examples and their writers

- `examples/rule_packs/invented_demo.yaml` and
  `fixtures/product_preview/invented_demo_rule_pack.json` move to `0.5.0`, with
  invented categories ("Invented category A") and required load cases naming
  the bundled demo model's ids. Their notices stay invented and non-engineering.
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
  - the pack aggregate follows FR-RULE-003 over the per-check statuses;
  - a single-basis pack gives the same computed value and status as the same
    check bound by row id before the change.
- **Desktop.** Manager editors and validation; the run panel shows each check's
  bases; the report's per-check projection is unchanged.
- **Native smoke.** The demo-pack steps still refuse as
  `RULE_INPUTS_INCOMPLETE` naming the missing inputs (to confirm).

## 6. Effects outside DEL-06

- **Reports** (`report_renderer`, `renderableReportInput.ts`, report package
  bindings): no change in this item. They project one status per check, which
  remains correct. Showing each check's bases in the report is a reports-lane
  follow-up.
- **Model document, analysis runs, handoff:** unchanged. They cite packs by id,
  version and checksum.
- **Solver:** unchanged. No product_physics file is touched.
