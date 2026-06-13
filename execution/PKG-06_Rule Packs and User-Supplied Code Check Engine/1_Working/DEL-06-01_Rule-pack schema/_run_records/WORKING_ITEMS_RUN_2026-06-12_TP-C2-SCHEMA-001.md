---
run-id: WORKING_ITEMS_RUN_2026-06-12_TP-C2-SCHEMA-001
timestamp: 2026-06-12T19:00:00-0600
completed: 2026-06-12T19:30:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/schemas
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C2-SCHEMA-001 — DEC-022 schema absorption (C1 residual closure, C2 lead-up)

## Tranche and authority basis

- Tranche: completion-plan Phase C item C2 lead-up — the named C1 residual
  "`schemas/rule_pack.schema.yaml` grammar_version/table additions"
  (`plans/PLAN_2026-06-10_prd_completion.md` §3 C1 row; TP-C1-GRAMMAR-001 run
  record "Open TBDs" item 1 in DEL-06-02 `_run_records/`).
- Binding authority: `DEC-022` (frozen typed AST grammar v1.0.0,
  `grammar_version` inside the JCS-hashed `rule_pack_checksum` payload), with
  `DEC-031` (accepted C1 ASSUMPTIONs) as companion context. No new decision
  was taken; this tranche records the already-ruled grammar into the schema.
- D-02b status at execution: packet prepared and AWAITING_RULING
  (2026-06-12); nothing in this tranche ships a writable text syntax. The
  schema carries only the declarative AST encoding.

## Changes

### `schemas/rule_pack.schema.yaml` (DEL-06-01 surface)

1. **Top-level `grammar_version` (required, strict semver pattern).** The
   member sits inside the JCS-canonicalized bytes hashed by
   `rule_pack_checksum`, satisfying the DEC-022 binding (the
   `core/rules/rule_pack_lifecycle` byte-containment check
   `payload_declares_grammar_version` finds the member in the canonical
   form).
2. **`FormulaDeclarationPayload.expression_ast` (optional property) + gates.**
   New `ExpressionNode` definition mirrors the frozen conformance-corpus node
   encoding (`fixtures/rule_expressions/conformance_corpus/README.md`):
   literal, variable_ref, unary (negate/abs/not), binary
   (add/subtract/multiply/divide), compare (6 operators), logical (and/or),
   select, aggregate (min/max, minItems 1), interpolate, lookup (exact/step).
   The evaluator refusal markers `unsupported_form` / `unsafe_host_access`
   are deliberately **not** authorable in pack documents. A conditional gate
   requires `expression_ast` and `grammar_status:
   frozen_open_pipe_stress_declared_expression` whenever `payload_kind` is
   `declarative_ast`; a sibling gate on `FormulaDeclaration` pins
   `expression_language: open_pipe_stress_declared_expression` whenever
   `declaration_form` is `declarative_ast`.
3. **`grammar_status` enum extended** with
   `frozen_open_pipe_stress_declared_expression` (legacy values retained for
   non-AST declaration forms).
4. **`UserTableValue` definition (D-02 §3 Q6 residual).** table_id,
   argument/result dimension + unit_ref, rows (minItems 1 of
   {argument, result} numbers). Monotonicity and the interpolate two-row
   floor stay evaluator-enforced (TableMalformed/TableOutOfRange) — JSON
   Schema cannot express them; the description says so explicitly.
5. **`ExpressionQuantity`** literal shape {value, dimension, unit_ref}. The
   corpus encoding's optional `unit_required`/`dimension_check_required`
   relaxation flags are deliberately not authorable in pack documents
   (stricter than the test-corpus encoding; evaluator defaults are `true`).
6. **`DimensionId` factored** out of `QuantityIntent.dimension` (identical
   28+TBD enum) and shared by `ExpressionQuantity` and `UserTableValue`.

### `examples/rule_packs/invented_demo.yaml` (DEL-06-05 surface)

- `schema_version` 0.1.0 → 0.2.0 (shape change: new required top-level
  member); `rule_pack_version` 0.1.0 → 0.2.0; top-level
  `grammar_version: "1.0.0"`.
- `demo_ratio_expression` upgraded from a prose-only `structured_expression`
  placeholder to a frozen-grammar `declarative_ast` payload:
  `divide(variable_ref demo_actual_quantity, variable_ref
  demo_limit_quantity)` — stress/stress with matching `demo_unit` unit refs,
  which the frozen evaluator derives as a dimensionless `ratio` quantity,
  matching the declared `output_dimension` exactly. All values remain
  invented non-engineering demonstration content.
- `open_decisions` rows OD-DEL-06-01-001 (expression_grammar) and
  OD-DEL-06-01-002 (evaluator_library) flipped deferred → accepted with
  DEC-022 pointers. Checksum block intentionally unchanged
  (`deferred_to_DEL_06_04` markers stay honest until the C2 backend seam
  lands a generator; next tranche).

### `examples/models/invented/fake_rule_pack_toy_model.json` (DEL-11-04 surface)

- `rule_pack_refs[0]`: version 0.1.0 → 0.2.0; raw-bytes checksum restamped
  `sha256:789acbd4fc7fc418cc79442ac0dac50a2fd45956b9316c5284cec0a3a0319fb2`;
  project JCS hash restamped
  `sha256:b0a7151adfcb9e932ef740d247307bc092d67e6ea57b2c7077f3ce77a7b53f2b`
  (both via the same helpers `tests/test_invented_example_models.py` uses).

### `tests/test_rule_pack_schema.py`

- Contract assertions for every addition above (including the
  refusal-marker exclusion and the relaxation-flag exclusion), example-shape
  assertions for the new payload, and seven new negative JSON Schema cases:
  missing/malformed `grammar_version`, declarative payload without
  `expression_ast`, stale `grammar_status` under `declarative_ast`,
  authored refusal marker, relaxed literal flags, malformed table row.

## Validation

- `python3 -m pytest -q tests` — **358 passed** (full root suite; includes
  the extended `test_rule_pack_schema.py` and the restamped
  `test_invented_example_models.py`).

## Residuals and hand-offs

- **Checksum stamping**: the example's `rule_pack_checksum` remains
  `deferred_to_DEL_06_04` markers; the C2 backend seam tranche
  (rule-pack lifecycle commands + JCS checksum generation) stamps real
  values and should add a Rust cross-check that evaluates the example's
  `expression_ast` through `core/rules/expression_evaluator` with invented
  bindings.
- **Check-evaluation composition** (surfaced TBD, not resolved here): how a
  check's pass/fail boolean is composed from `formula_ref` output and
  `value_slot_refs` limits (e.g. synthesized `Compare(formula, <=, slot)`)
  is C4 wiring scope. The schema deliberately does not freeze it; PRD §12.5
  shows `expression` + `allowable` as separate check fields.
- **Load-case mapping vocabulary** (surfaced TBD, not resolved here): PRD
  §12.2 lists "Load-case mappings" among rule-pack capabilities;
  `CheckDefinition` has no combination/load-case mapping member yet. Routed
  to the C4 lead-up (it binds checks to solved combinations at run time).
- DEL-06-02/DEL-06-04 production documents still carry pre-DEC-022 grammar
  TBD prose; resolvable at formal review per the C1 run record (unchanged
  here).

## Boundary review

- No protected standards content, no private data, no code-specific values:
  every value remains invented demonstration content with provenance.
- No lifecycle state, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim is
  created. Schema and example changes are draft/proposal authority only.
