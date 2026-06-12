# Expression Evaluator

This crate is the bounded implementation slice for `DEL-06-02`. It evaluates a
declarative expression tree supplied by governed rule-pack/application
boundaries. It does not parse arbitrary text, execute host-language code, load
plugins, access files, access the network, spawn processes, provide protected
standards content, or make professional/code-compliance claims.

Per human ruling **DEC-022** (D-02 Option A, 2026-06-11), the typed AST in
this crate is the **frozen canonical rule-pack expression grammar**
(`expression_language: open_pipe_stress_declared_expression`), versioned by
`GRAMMAR_VERSION` (`1.0.0`). A writable text syntax is deferred to a named
follow-up ruling (D-02b); no parser exists at this freeze.

## Scope (frozen function set, grammar v1.0.0)

- Numeric literals and explicit variable bindings with dimension and unit
  reference metadata.
- Arithmetic (`add`, `subtract`, `multiply`, `divide`, `negate`, `abs`) with
  enumerated dimension-product algebra over the closed `Dimension` enum:
  dimensional products/quotients resolve only through the in-crate
  `DIMENSION_PRODUCTS` table; unrepresentable or ambiguous results are
  blocking findings, never silent enum extension. Derived unit references are
  composed deterministically (`a*b` sorted for products, `a/b` for quotients,
  `ratio` for dimensionless) — this crate still owns no unit conversion
  (DEC-018 places conversion at the units-catalog boundary).
- The six comparisons over compatible dimensions with matching unit refs.
- Boolean `and`/`or`/`not` and the eager `select` conditional (all
  subexpressions always evaluated; diagnostics in unselected branches block).
- N-ary `min`/`max` over same-dimension, same-unit quantities.
- Piecewise-linear `interpolate` and exact/step `lookup` over user-supplied
  monotone tables; out-of-range arguments are blocking diagnostics — no
  extrapolation and no clamping, ever.
- Grammar-version gating: the rule pack's declared `grammar_version` (semver,
  also bound inside the JCS-hashed rule-pack checksum payload, see
  `core/rules/rule_pack_lifecycle`) must be in `SUPPORTED_GRAMMAR_VERSIONS`,
  else evaluation blocks with `UnsupportedGrammarVersion`.
- Deterministic findings for unsafe constructs, unsupported expression forms,
  missing variables, duplicate bindings, invalid references, non-finite
  values, division by zero, missing unit metadata, unit mismatches, dimension
  mismatches, type mismatches, missing required values, malformed tables,
  out-of-range/missed table lookups, unsupported grammar versions, and
  analysis-status boundary violations — in fixed evaluation order.
- Analysis-status preservation for mechanics solved, rule-input incomplete,
  user-rule checked, user-rule failed, and human-review-required states.

## Boundary

The evaluator accepts structured expression data only. Interpolation/lookup
*mechanisms* are public; table *values* are user value-slots and never
repository content. `power`/`sqrt` are excluded from v1.0.0 and recorded as a
named open candidate for v1.1 (DEC-022). This crate does not implement the
required-input completeness checker, private rule-pack lifecycle, checksum
handling, public example rule packs, GUI editors, report generation, public
API transport, private storage, unit conversion constants, or final
result-envelope integration. Same-dimension arithmetic and comparisons
require matching unit references because this crate does not own unit
conversion.

## Verification

- In-crate unit tests cover the full function set, dimension algebra
  (including ambiguity blocking and product-table key uniqueness), grammar
  version gating, table diagnostics (including non-finite rows that JSON
  cannot encode), eager-select semantics, and the pre-freeze behaviors
  (sandbox refusals, status boundary, binding diagnostics).
- The blessed golden conformance corpus
  `fixtures/rule_expressions/conformance_corpus/` (69 cases, invented values
  only) runs via `tests/conformance_corpus.rs` under plain `cargo test` with
  an enforced coverage floor: every function kind has an accepting case and
  every relevant finding code has a blocking case. The corpus is the freeze
  artifact: grammar changes require a corpus extension and a version bump.
