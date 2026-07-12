# MEMORY - DEL-06-02 Sandboxed Unit-Aware Expression Evaluator

## 2026-07-12 - D41-R5-T2B-PDU024 downstream version integration

- Recorded the bounded downstream integration for PDU-024: project-carried evaluator inputs consume DEL-02-05's accepted `0.2.0` model-document family and explicit current/stale/unsupported/newer/failed version-check behavior.
- No expression grammar, variable/result binding contract, evaluator library, sandbox behavior, rule-pack fixture, lifecycle state, dependency/DAG/register, protected content, private data, or professional claim changed.
- Shared focused desktop evidence passed 65/65 tests; this deliverable claims supporting integration evidence only.

## 2026-06-18 - TP-UNITS-BTAIL-RULEPACKEXPRLINTUNITS-001 supporting report-lint inventory evidence

- Supporting role for DEL-06-02: Report Content Lint now inventories the
  existing Rule Pack Expression Composer unit-policy surface,
  `rule-pack-expression-unit-policy`.
- The lint packet now includes
  `apps/desktop/src/features/rule-packs/ExpressionComposer.tsx` and records
  the expression AST unit-reference policy as public inventory evidence.
  Public unit-policy targets increase from 36 to 37; conversion-witness
  targets remain two and lint conversion remains false.
- This is inventory over existing editor-side expression unit refs only. It
  does not change evaluator normalization, exact-unit matching, blocking
  findings, grammar, parser policy, run-check execution behavior, or backend
  validation.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULEPACKEXPRLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-254; completion log entry; primary
  DEL-08-05 run record and supporting DEL-07-03/DEL-02-02 run records.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip tests; focused ExpressionComposer/RulePackManagerPanel Vitest
  6/6; focused R2/rule-pack Playwright smoke 4/4; full desktop Vitest
  399/399; single-worker R2/R3 Playwright smoke 18/18; and desktop production
  build with the existing Vite large-chunk warning.
- Boundary preserved: no evaluator behavior, rule-pack schema, expression
  grammar, writable text parser/syntax, backend validation, persistence, unit
  conversion API, DEC-018 catalog constant, schema dimension enum, protected
  standards content, private data, lifecycle transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## Implementation Summary

Implemented the bounded `core/rules/expression_evaluator` Rust crate for the
sealed `DEL-06-02` scope.

The crate evaluates explicit declarative expression trees with:

- quantity literals carrying dimension metadata;
- variable bindings from rule-pack required inputs, user-supplied values, or
  solver result fields;
- unary negation;
- addition and subtraction over matching dimensions;
- dimensionless scaling;
- same-dimension division to dimensionless ratios;
- same-dimension comparisons;
- deterministic findings for unsafe constructs, unsupported forms, missing or
  duplicate bindings, invalid references, missing required values, non-finite
  inputs, division by zero, dimension mismatches, type mismatches, and
  analysis-status boundary violations.

The implementation intentionally does not add a parser, third-party expression
library, host-language evaluation, filesystem access, network access, process
execution, plugin loading, private rule-pack lifecycle, checksum handling,
completeness checking, GUI behavior, report generation, or public API
transport.

## Verification

Focused tests cover:

- successful invented expression evaluation;
- unsafe construct rejection;
- unsupported expression forms;
- missing variable binding;
- duplicate binding;
- invalid reference;
- missing required value;
- non-finite literal;
- division by zero;
- dimension mismatch;
- dimensionless scaling;
- dimensional multiplication remaining unsupported pending derived-dimension
  policy;
- same-dimension ratio output;
- human approval status boundary handling.

## Open Items

- Final expression grammar/library selection remains `TBD`.
- Parser dependency policy remains `TBD`.
- Complete quantity representation and conversion constants remain `TBD`.
- Comparison tolerance policy remains `TBD`.
- Final diagnostic code taxonomy and result-envelope integration remain `TBD`.
- Variable namespace and result-field binding contract remain `TBD`.
- Threat-model review depth remains `TBD`.
- GUI editor presentation, private storage, checksum lifecycle, report
  integration, and public API transport remain downstream deliverables.

## 2026-05-11 TP-RECON-01 Reconciliation

Reconciled DEL-06-02 history from the TP-RECON-01 dispatch row, archived
DEV-001 evidence files, archived lifecycle snapshot, archived DEL-06-02
dispatch brief, SCA-002 coordination notes, current deliverable run records,
and commit `7490f67`.

Evidence records identify the implemented slice as `BACKEND_FEATURE_SLICE`
scope `SOW-045` / objective `OBJ-005`, committed on 2026-05-02 as
`7490f67` (`core: add rule expression evaluator`) with handoff commit
`36bfb25`. The commit added the bounded
`core/rules/expression_evaluator` crate, updated `docs/SPEC.md` and
`docs/TYPES.md`, added this deliverable memory record, and updated bounded
coordination state.

The reconciled implementation surface remains the sandboxed deterministic
expression-tree evaluator: quantity literals with dimension metadata,
explicit variable bindings, unary negation, matching-dimension arithmetic,
dimensionless scaling, same-dimension ratios and comparisons, deterministic
findings for invalid or unsafe inputs, and preservation of analysis-status
boundaries. Archived verification records `rustfmt`, `rustc --test`, 14
focused evaluator tests, and `git diff --check` as passing during closeout.

Lifecycle evidence remains `CHECKING`, not release or reliance state. Deferred
scope remains final grammar/library selection, parser dependency policy,
complete quantity representation and conversion constants, comparison
tolerance, diagnostic/result-envelope integration, variable namespace and
result-field binding, threat-model depth, GUI editor presentation, private
storage, checksum lifecycle, report integration, and public API transport.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator/_REVIEW.md` and `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator/Review_Findings.csv`.
- Package audit summary is `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/_run_records/TASK_RUN_2026-05-16_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 1 (WARNING=1). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=1.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-06-02`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-05 - Human disposition accepted technical resolution

- Human project authority accepted the technical resolution for
  `PKG06-02-PKG02-001`.
- `Review_Findings.csv` now records `HumanDisposition=ACCEPT_AS_IS` and
  `Status=RESOLVED`.
- This closes the local review-finding gate only; `_STATUS.md`, release
  readiness, and professional/code-compliance boundaries are unchanged.

## 2026-06-05 - PKG06-02-PKG02-001 verification

- Bounded TASK verification confirmed `core/rules/expression_evaluator` still carries explicit `Quantity` unit metadata (`unit_ref`, `unit_required`, `dimension_check_required`) and still rejects unsafe host-access forms through deterministic findings.
- `cargo fmt --manifest-path core/rules/expression_evaluator/Cargo.toml --check` passed.
- `cargo test --manifest-path core/rules/expression_evaluator/Cargo.toml --locked` passed with 17 unit tests and 0 doctests.
- Finding `PKG06-02-PKG02-001` remains technically addressed and ready for human disposition; no lifecycle/status, dependency, review-finding, source, schema, test, DAG, or coordination files were changed.

## 2026-06-11 - TP-C1-GRAMMAR-001: DEC-022 grammar freeze implemented

- Human ruling `DEC-022` (D-02 Option A) implemented: the typed AST in
  `core/rules/expression_evaluator` is now the frozen canonical rule-pack
  expression grammar (`open_pipe_stress_declared_expression`, grammar version
  `1.0.0`; no text parser — D-02b deferred). Added boolean And/Or/Not, eager
  Select, n-ary Min/Max, Abs, piecewise-linear Interpolate and exact/step
  Lookup over user-supplied monotone tables (out-of-range = blocking, no
  extrapolation/clamping), and enumerated dimension-product algebra resolving
  the two long-standing in-code dimensional TBDs (unrepresentable/ambiguous
  products block; closed `Dimension` enum never silently extended).
- `grammar_version` (strict semver) is gated in the evaluator
  (`UnsupportedGrammarVersion` blocking finding) and bound into the
  JCS-hashed rule-pack checksum seam in `core/rules/rule_pack_lifecycle`
  (record field, blocking findings, byte-containment evidence check,
  binding-enforcing checksum constructor).
- Freeze artifact: blessed golden conformance corpus
  `fixtures/rule_expressions/conformance_corpus/` (69 invented-value cases,
  enforced coverage floor, executed by plain `cargo test` in this crate) plus
  `fixtures/rule_expressions/checksum_binding/` golden-hash corpus in the
  lifecycle crate. Grammar changes require corpus extension + version bump.
- Validation: evaluator 31 unit tests + corpus runner (69 cases) pass;
  lifecycle 12 pass; completeness_checker 12 pass (regression);
  `python3 -m pytest -q tests` 353 pass; `cargo fmt --check` clean.
- REQ-06-02-006 / OI-006 ("grammar remains TBD") are now resolvable against
  DEC-022 at formal review; production documents intentionally not edited.
  Surfaced handoffs (out of scope): `schemas/rule_pack.schema.yaml` needs
  `grammar_version` + frozen `grammar_status` enum value + table value
  structure; decomposition OI-006/DEC-012 TBD retirement is a coordination
  update. Unit semantics remain exact-string match pending Phase B1 catalog.
- Run record: `_run_records/TASK_RUN_2026-06-11_TP-C1-GRAMMAR-001.md`
  (includes review-attention items: drafted dimension-product table contents,
  derived unit-ref composition convention, Select branch compatibility rule).
- No lifecycle promotion, release claim, or professional/code-compliance
  claim is implied; evidence is `CHECKING` state only.

## 2026-06-12 - DEC-031: the three C1 grammar ASSUMPTIONs ruled accepted

- Human ruling `DEC-031` (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12,
  companion disposition 2 of the D-14 sitting) accepted all three labeled
  ASSUMPTIONs from `TP-C1-GRAMMAR-001` as drafted:
  1. the enumerated `DIMENSION_PRODUCTS` closed-enum table contents
     (18 standard mechanics relations, e.g. Force×Length→Moment,
     Stress×SectionModulus→Moment, α×ΔT→dimensionless);
  2. derived unit-reference composition for dimensional products/quotients
     (`a*b` sorted / `a/b` / `ratio`) per the crate's ratio-synthesis
     precedent, pending Phase B conversion integration;
  3. the conservative Select branch-compatibility freeze (branches must be
     type/dimension/unit-compatible even though only one branch is returned).
- These are now ruled dispositions for formal review, no longer open
  assumptions. Grammar v1.0.0 content unchanged; no code edit was needed.
- No lifecycle promotion, release claim, or professional/code-compliance
  claim is implied; evidence is `CHECKING` state only.

## 2026-06-14 - TP-C4-ACCEPTREL-001: acceptability relation beyond `<=`

- The rule-check runner (`core/rules/rule_check_runner`) no longer hard-codes
  `<=` for the top-level computed-vs-limit acceptability comparison. It reads an
  optional `acceptability_relation` on each `CheckDefinition` (four ordering
  relations `less_than` / `less_than_or_equal` / `greater_than` /
  `greater_than_or_equal`), maps it to the frozen grammar's existing
  `ComparisonOperator` (DEC-022; no grammar change), and reports the relation on
  the outcome.
- Backward compatible: **absent → `less_than_or_equal`** (the demo and every
  pre-member pack behave identically — witnessed by the unchanged
  `invented_demo_run` integration tests). An explicit but **unrecognized token
  blocks** the check (`RULE_EVALUATOR_ERROR` → `RULE_INPUTS_INCOMPLETE`), never a
  silent `<=`. Equality acceptance is a deliberate non-goal of the member.
- Additive **PROPOSAL** schema member (`schemas/rule_pack.schema.yaml`
  `CheckDefinition.acceptability_relation`), awaiting human ratification
  (companion to `DEC-038`; ratification bumps rule-pack `schema_version`
  0.3.0 → 0.4.0 per `DEC-033`). Authored via the C2 check-definitions selector
  (DEL-07-03); the draft template authors the explicit default.
- Evidence: run record
  `_run_records/WORKING_ITEMS_RUN_2026-06-14_TP-C4-ACCEPTREL-001.md`; SMOKE
  TP-MAC-163. Runner `cargo test` 18; pytest schema 5; Vitest 367; e2e 2/2.
- No lifecycle promotion, release claim, or professional/code-compliance
  claim is implied; evidence is `CHECKING` state only.

## 2026-06-15 - TP-C4-SOLVERREFPICKER-001: run-panel preview for authored `solver_result_ref`

- The run-rule-checks panel now preserves and recognizes a `solver_result`
  input's authored `solver_result_ref` (`{result_id}`) in the binding plan.
  Inputs with that member are treated as canonical in-pack references and no
  longer render or send the legacy caller-supplied result-row selector for that
  input; packs without the member keep the selector path.
- Added a read-only "Preview result row" action that classifies the authored
  result id against the current solved envelope as `resolves`, `result_missing`,
  or `no_result_rows`, lists available rows when present, and surfaces that
  unresolved references block at `RULE_INPUTS_INCOMPLETE` rather than receiving a
  fallback binding.
- Evidence: run record
  `_run_records/WORKING_ITEMS_RUN_2026-06-15_TP-C4-SOLVERREFPICKER-001.md`;
  SMOKE TP-MAC-167; focused Vitest 27/27, full desktop Vitest 378/378,
  production build clean, focused Playwright run-rule-checks 2/2 after correcting
  the browser-preview expectation to `no_result_rows`.
- No schema/backend/lifecycle/decision-register change; no private value or
  protected content embedded; no release, professional, certification, sealing,
  authentication, approval, or code-compliance claim is implied.

## 2026-06-15 - TP-UNITS-B2-RULEEXPRUNITS-001: expression composer unit selectors

- The C2 expression composer now offers DEC-018-backed desktop selectors for
  existing AST unit metadata: literal `quantity.unit_ref`, table
  `argument_unit_ref`, and table `result_unit_ref`.
- The selector path is UI-only and dimension-filtered; it keeps
  out-of-catalog stored units visible instead of silently rewriting the
  rule-pack document. Browser preview remains manual unit text entry because
  the reviewed unit catalog is desktop/Tauri-only.
- Evidence: run record
  `_run_records/WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-RULEEXPRUNITS-001.md`;
  corresponding DEL-02-02 run record; SMOKE TP-MAC-169; focused
  `ExpressionComposer` Vitest 19/19, full desktop Vitest 384/384, production
  build clean, Playwright e2e 10/10.
- No schema/backend/evaluator/parser/text-syntax/lifecycle change; no private
  value or protected content embedded; no release, professional,
  certification, sealing, authentication, approval, or code-compliance claim is
  implied.

## 2026-06-16 - TP-UNITS-B2B3-RULECHECKNORM-001: rule-check mixed-unit normalization

- `core/rules/rule_check_runner` now normalizes compatible DEC-018 catalog
  units to the rule-pack declaration unit before invoking the exact-unit
  expression evaluator. This keeps the frozen evaluator semantics intact while
  allowing authored packs with `Pa` declarations to accept compatible runtime
  values entered as `MPa` or `kPa`.
- Exact non-catalog demonstration units remain valid only when entered and
  declared strings match. Unknown or incompatible substitutions block with a
  `UnitMismatch` finding and `RULE_INPUTS_INCOMPLETE`; no value is silently
  coerced or evaluated.
- Desktop command coverage proves the behavior through `run_rule_checks_core`
  with a re-stamped invented demo pack (`0.05 MPa` actual, `100 kPa` limit,
  declared `Pa`).
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-RULECHECKNORM-001.md`;
  supporting DEL-02-02 run record; SMOKE TP-MAC-176. Validation: runner cargo
  tests (13 unit + 7 integration), desktop Tauri cargo tests 62/62, desktop
  Vitest 386/386, desktop build.
- No grammar, schema, parser, lifecycle, release, professional, certification,
  sealing, authentication, approval, or code-compliance claim is implied.

## 2026-06-16 - TP-UNITS-B2-RULECHECKRUNUNITS-001 run-check binding unit controls

- Added supporting B2/B3 unit evidence for the GUI side of C4 run-time
  user-supplied rule-check value bindings.
- The run-check panel now uses DEC-018-backed desktop selectors for runtime
  value and value-slot binding units while preserving browser manual text
  entry. The runner payload remains explicit `{value, unit, dimension}` and
  the evaluator/normalization semantics are unchanged.
- Evidence:
  `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-04_Missing-data warning and blocking UX/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-RULECHECKRUNUNITS-001.md`;
  supporting run record in this deliverable; SMOKE TP-MAC-182; completion log
  entry.
- Validation passed: focused RuleCheckRunPanel Vitest 18/18; full desktop
  Vitest 389/389; desktop build; focused/full Playwright; in-app Browser
  verification; DEC-025 dirty-tree sweep pass.
- Boundary unchanged: no schema, evaluator, grammar, parser, writable
  expression text syntax, protected content, private data, release-readiness
  claim, or professional/code-compliance claim changed.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-RULEPACKUNITPOLICY-001 supporting evaluator evidence

- Added supporting evidence for the DEL-07-03 rule-pack expression authoring
  surface's visible unit policy summary over existing expression AST unit refs:
  literal quantities and table argument/result unit metadata.
- The evaluator contract is unchanged. The summary preserves stored unit refs,
  records `conversion=false`, and reports browser/catalog validation status
  before runtime without changing normalization, exact-unit matching, blocking
  findings, grammar, parser, or run-check execution behavior.
- Evidence:
  `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RULEPACKUNITPOLICY-001.md`;
  supporting run record in this deliverable; SMOKE TP-MAC-207; completion log
  entry.
- Validation passed: focused rule-pack/unit Vitest 67/67, focused R2/R3
  Playwright smoke file 14/14, full desktop Vitest 398/398, and desktop
  production build with the existing Vite large-chunk warning.
- Boundary unchanged: no evaluator normalization, grammar, schema, parser,
  writable expression text syntax, backend validation/persistence behavior,
  protected content, private data, release-readiness claim, or professional/
  code-compliance claim changed.
