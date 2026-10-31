# Load Case Algebra

This crate is the bounded implementation slice for `DEL-05-02`. It combines
explicit mechanics operands from primitive load cases or result states without
providing protected design-code combinations, public default factors, rule-pack
expression evaluation, stress recovery, or professional/code-compliance claims.

## Scope

- User-defined linear combination terms with explicit numeric factors.
- Result-state subtraction and min/max range envelopes over compatible
  mechanics quantities.
- Unsigned combinations over explicit factored terms (`evaluate_absolute_sum`,
  `evaluate_srss`; DEL-05-02 Work `absolute-sum-srss-combinations.md` §3): the
  absolute sum RN(Σ|fᵢ·xᵢ|), one rounding of the exact sum, and the SRSS
  RN(√Σzᵢ²) with zᵢ = RN(fᵢ·xᵢ), one rounding of the exact root, found by exact
  midpoint signs on the exact accumulator. Both are +0.0 for an exact zero and
  independent of term order; a result outside the binary64 range, an
  overflowing zᵢ or a non-finite operand value is a `NonRepresentable` finding,
  never an infinity; more than 2³² terms are refused by name.
- Dimension compatibility checks using the existing primitive-load dimension
  vocabulary.
- Result boundary records that require explicit unit metadata, provenance
  reference, result-schema binding, JCS payload reference, and payload-hash
  reference before algebra quantities are handed to persistence or result
  envelopes.
- Analysis-status preservation for mechanics solved, rule-input incomplete,
  user-rule checked, user-rule failed, and human-review-required states.
- The unit factor-sum rule shared by the combination gates
  (`factor_sum_within_unit_tolerance`): the exact sum of the binary64 factors,
  admitted iff |Σf − 1| ≤ 64ε, compared exactly (DEL-04-07
  `Design/pressure-contract.md`, "Constant-effort combinations").
- Deterministic findings for missing operands, non-finite factors, unsupported
  expression shapes, dimension mismatches, duplicate operands, missing result
  states, and professional-boundary violations.

## Boundary

This crate does not implement a general expression parser, the rule-pack
expression evaluator, code-specific load-combination defaults, wind/seismic
procedures, stress-recovery formulas, protected standards content, proprietary
project data, report rendering, GUI behavior, or headless execution.

Inputs are explicit mechanics/result quantities that upstream schemas, units,
provenance, and load/status boundaries must already govern. Unsupported
semantics remain deterministic findings or `TBD` decisions, not silent
defaults.

The algebra API rejects load-record schema bindings for result quantities.
Successful result boundary records bind to the canonical model result value or
schema-first result-envelope quantity surface and preserve the existing
dimension checks plus explicit unit metadata. This crate records payload and
hash references only; it does not compute hashes, choose conversion constants,
or define code-specific calculation units.

## Verification

The unit tests cover linear combinations, result-state subtraction, range
envelopes, dimension compatibility, missing/invalid operands, duplicate
operands, result boundary metadata, status propagation, and rejection of
human-approval status as an automatic software result. `src/unsigned_tests.rs`
checks the absolute sum and the SRSS against the frozen table in
`validation/references/rt_a4_combinations/` bit for bit (V-1), against
`correct_norm::norm3` on a random sample (V-2), over every order of a
five-term table (V-3), and against the mutants of V-4.
