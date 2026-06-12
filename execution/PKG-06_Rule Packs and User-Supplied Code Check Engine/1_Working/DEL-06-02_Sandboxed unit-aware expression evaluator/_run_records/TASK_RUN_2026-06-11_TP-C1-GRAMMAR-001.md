---
run-id: TASK_RUN_2026-06-11_TP-C1-GRAMMAR-001
timestamp: 2026-06-11T21:30:00-0600
completed: 2026-06-11T21:47:41-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/core/rules
task-profile: NONE
task-skill: NONE
resolved-skill-path: NONE
resolved-skill-version: UNKNOWN
resolved-task-profile-requirement: NONE
companion-files:
  - none checked
allowed-tools:
  - unrestricted
write-authorization: EXPLICIT_BRIEF_TEXT
runtime-overrides: {}
---

# TASK_RUN TP-C1-GRAMMAR-001 — DEC-022 expression grammar freeze implementation

## Tranche and authority basis

- Tranche: completion-plan Phase C item C1 (`plans/PLAN_2026-06-10_prd_completion.md` §3 C1).
- Binding authority: human ruling `DEC-022` (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 row DEC-022, 2026-06-11), selecting Option A of decision packet `execution/_Coordination/_DECISIONS/D-02_rule_pack_expression_grammar.md`.
- Run-record location: this deliverable (DEL-06-02) owns `core/rules/expression_evaluator` per `_CONTEXT.md` ("rule evaluator module"); checksum-seam changes also touch `core/rules/rule_pack_lifecycle` (DEL-06-04 scope) and are reported here as the dispatching brief directed a single run record at the evaluator-owning deliverable.

## Requested Tasks

1. Extend the typed expression AST in `core/rules/expression_evaluator` to the full PRD §12.3 function set per DEC-022 (And/Or/Not, eager Select, n-ary Min/Max, Abs, piecewise-linear Interpolate, exact/step Lookup over user-supplied monotone tables with out-of-range blocking).
2. Resolve the evaluator's two in-code dimensional TBDs (dimensional multiply/divide) with dimension-product algebra.
3. Bind `grammar_version` (semver) inside the JCS-hashed rule-pack checksum payload at the in-scope seam (`core/rules/rule_pack_lifecycle`); surface out-of-scope schema changes as handoffs.
4. Land a blessed golden conformance corpus under `fixtures/rule_expressions/` (synthetic values only) wired into crate tests as executable evidence.

## Expected Outputs

- Extended evaluator + lifecycle crates with passing `cargo test`.
- `fixtures/rule_expressions/` conformance corpus + README provenance.
- This run record and a dated `MEMORY.md` section.

## Tools Used

- zsh cargo (fmt, test)
- zsh python3 (corpus file generation, inventory table generation; generator was a one-shot `/tmp` script, removed after use)
- zsh shasum (golden hash derivation for checksum-binding fixtures)
- zsh git/grep/find/sed/date (read-only inspection and boundary checks)

## Tool Policy Compliance

N/A — no tool allowlist was active. All writes stayed inside the brief-authorized targets (verified against `git status` at closeout; concurrent unrelated worktree changes by other tranches were left untouched).

## Write Authorization

- Brief-authorized write targets: `core/rules/**`, `fixtures/rule_expressions/**` (new), rule-expression-specific `tests/test_*` files (none were needed — no Python surface references these fixtures and `schemas/` is unchanged), this `_run_records/` file, and a dated `MEMORY.md` section in this deliverable.
- Hard exclusions honored: no `docs/` governance, no `schemas/`, no `apps/**`, no `core/**` outside `core/rules`, no `plans/**`, no `execution/_Coordination/**`, no `execution/_Decomposition/**`, no git commit/push, no network, no new dependencies (the corpus runner uses a dependency-free strict JSON reader inside the test harness; both crates remain zero-dependency).

## Outputs Produced

### `core/rules/expression_evaluator` (DEL-06-02)

- Grammar freeze surface: `GRAMMAR_VERSION = "1.0.0"`, `SUPPORTED_GRAMMAR_VERSIONS`, `grammar_version_supported()`; `EvaluationInput.declared_grammar_version` gate producing blocking `FindingCode::UnsupportedGrammarVersion` for undeclared/malformed/unsupported versions (never best-effort).
- Function set (frozen v1.0.0): `UnaryOperator::{Abs, Not}` added; new `Expression::{Logical (And/Or), Select (eager), Aggregate (n-ary Min/Max), Interpolate, Lookup (Exact/Step)}` with `UserTable`/`TableRow`/`LookupMode`. Eager semantics: all subexpressions always evaluated in fixed order; out-of-range table arguments are blocking `TableOutOfRange` (no extrapolation, no clamping); in-range exact-lookup misses are `TableKeyNotFound`; structural table problems are `TableMalformed`.
- Dimension-product algebra resolving the two in-code TBDs (former lib.rs:549 multiplication, :591 division): enumerated commutative `DIMENSION_PRODUCTS` table (18 mechanics relations) over the closed `Dimension` enum, with `dimension_product()` / `dimension_quotient()` (quotients derived by inverting the table; ambiguous quotients such as Force/Area — Pressure vs Stress — block). Unrepresentable/ambiguous results are blocking `UnsupportedExpressionForm`; the closed enum is never silently extended. Derived unit refs are composed deterministically (`a*b` lexicographically sorted, `a/b`, `ratio` for dimensionless), consistent with DEC-018 (no in-evaluator conversion).
- Tests: 31 in-crate unit tests (was 17) + `tests/conformance_corpus.rs` golden-corpus runner with enforced coverage floor and determinism re-run check.
- README rewritten for the frozen scope.

### `core/rules/rule_pack_lifecycle` (DEL-06-04 seam)

- `GRAMMAR_VERSION_PAYLOAD_KEY`, `grammar_version_jcs_member()`, `payload_declares_grammar_version()` (byte-containment evidence check — documented as a necessary-condition check, not JSON parsing, preserving the crate's no-JSON-parsing boundary), `is_semver_grammar_version()`.
- `RulePackLifecycleRecord.grammar_version` (new field, surfaced in `AuditManifestEntry`); blocking lifecycle findings `MissingGrammarVersion`, `GrammarVersionMalformed`, `GrammarVersionNotBound`; `LifecycleValidationInput.canonical_payload` for the binding check (applied only to `RulePackPayload`-scope checksums).
- `ChecksumRecord::sha256_caller_supplied_jcs_bytes_with_grammar_version()` binding-enforcing constructor with `GrammarVersionBindingError`.
- Tests: 12 (was 8), including a golden fixture-corpus test.
- README updated.

### `fixtures/rule_expressions/` (new)

- `README.md` (provenance: all values invented, `PUBLIC_DOMAIN_OR_ORIGINAL`; freeze rule).
- `conformance_corpus/`: 69 `case_*.json` golden cases + README (case format, node encoding, coverage floor, bless/quarantine workflow, inventory). Executed by `cargo test` in the evaluator crate; coverage floor enforced in the runner: every function kind/operator has an accepting case, every relevant `FindingCode` has a blocking case (`non_finite_input` excluded — not JSON-encodable; pinned by unit tests), grammar-version accept/block pinned, corpus must not shrink below 60 cases.
- `checksum_binding/`: three invented JCS-style payloads (with `grammar_version` 1.0.0, with 1.1.0, without) + `MANIFEST.tsv` golden SHA-256 values + README. Executed by `cargo test` in the lifecycle crate; pins that changing/removing the grammar-version member changes the rule-pack checksum.

## Validation evidence (all passing)

- `cargo fmt --check` clean on both touched crates.
- `cargo test --manifest-path core/rules/expression_evaluator/Cargo.toml --locked`: **31 unit tests + 1 corpus runner test (69 cases) passed, 0 failed**.
- `cargo test --manifest-path core/rules/rule_pack_lifecycle/Cargo.toml --locked`: **12 passed, 0 failed**.
- `cargo test --manifest-path core/rules/completeness_checker/Cargo.toml --locked` (untouched neighbor, regression check): **12 passed, 0 failed**.
- `python3 -m pytest -q tests`: **353 passed** (no Python surface was modified; run to confirm the new fixtures directory breaks nothing).
- All 69 corpus expectations were hand-derived from the frozen semantics and passed on first execution (no expectation was adjusted to match the implementation).

## Missing

- none

## Needs Human Ruling

- none blocking. The following implementation choices were made inside the DEC-022 authority, are pinned by the corpus, and deserve routine human review attention:
  - ASSUMPTION: the enumerated `DIMENSION_PRODUCTS` table contents (18 standard mechanics relations, e.g. Force x Length -> Moment, Stress x SectionModulus -> Moment, alpha x deltaT -> dimensionless). DEC-022 ruled dimension-product algebra resolving the two TBDs; the packet's §5.3 enumerated-closed-enum approach was followed, but the specific relation list is drafted here, not enumerated in the ruling.
  - ASSUMPTION: derived unit-reference composition for dimensional products/quotients (`a*b` sorted / `a/b` / `ratio`), following the crate's existing `ratio` synthesis precedent; the ruling did not specify derived unit_ref representation (no conversion exists until Phase B1 lands).
  - ASSUMPTION: `Select` branches must be type/dimension/unit-compatible even though only one is returned (conservative freeze; eager semantics already evaluate both).

## Dependency Notes

- DEC-018 / Phase B1: evaluation still uses exact-string unit refs with no conversion; the B1 units-catalog crate remains the conversion boundary. Corpus unit semantics pin current exact-match behavior, per the packet's B1-lag sequencing note.
- D-04 tolerances: all corpus expected values are exact-match on canonical outputs (values chosen exact in binary floating point); no tolerance was invented.

## Handoffs surfaced (out of write scope — not performed)

- TBD (schemas/): `schemas/rule_pack.schema.yaml` needs `grammar_version` (required semver) in the pack payload / `FormulaDeclarationPayload`; a `grammar_status` enum value naming the frozen grammar (current enum has only `grammar_not_selected` / `future_human_approved_grammar_required` / `TBD`); and a user-table value structure for Interpolate/Lookup (D-02 §3 Q6). `expression_language` already contains `open_pipe_stress_declared_expression`.
- TBD (deliverable docs): DEL-06-02 `Specification.md` REQ-06-02-006 and `Guidance.md` OI-006 ("grammar remains TBD") are now resolvable against DEC-022 at this deliverable's formal review; production documents were not edited by this run.
- TBD (coordination/decomposition): retiring the "rule expression grammar/library" member of the DEC-012 TBD set and OI-006 in `SOFTWARE_DECOMP.md` is a coordination-surface update outside this brief.

## Conflicts found (reported, not resolved)

- No conflict between DEC-022 and existing code semantics. One interpretation note: the DEC-022 register row resolves D-02 packet §3 Q2 (dimension-algebra mechanism) only implicitly; this run implemented the packet recommendation §5.3 mechanism (enumerated product table over the closed enum, blocking unrepresentables) as the only zero-invention reading consistent with the row text. Recorded above under Needs Human Ruling review-attention items.

## Applied Changes

- `core/rules/expression_evaluator/src/lib.rs` (extended; 2 pre-existing TBD branches removed per ruling)
- `core/rules/expression_evaluator/tests/conformance_corpus.rs` (new)
- `core/rules/expression_evaluator/README.md` (rewritten)
- `core/rules/rule_pack_lifecycle/src/lib.rs` (extended)
- `core/rules/rule_pack_lifecycle/README.md` (updated)
- `fixtures/rule_expressions/README.md` (new)
- `fixtures/rule_expressions/conformance_corpus/` (new; README + 69 case files)
- `fixtures/rule_expressions/checksum_binding/` (new; README + MANIFEST.tsv + 3 payload files)
- This run record; dated section appended to this deliverable's `MEMORY.md`.

## Proposed Changes

- none beyond the handoffs surfaced above.
