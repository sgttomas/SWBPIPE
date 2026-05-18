# Expression Evaluator

This crate is the bounded implementation slice for `DEL-06-02`. It evaluates a
small declarative expression tree supplied by governed rule-pack/application
boundaries. It does not parse arbitrary text, execute host-language code, load
plugins, access files, access the network, spawn processes, provide protected
standards content, or make professional/code-compliance claims.

## Scope

- Numeric literals and explicit variable bindings with dimension and unit
  reference metadata.
- Arithmetic over compatible or explicitly dimensionless quantities.
- Comparisons over compatible dimensions.
- Deterministic findings for unsafe constructs, unsupported expression forms,
  missing variables, duplicate bindings, invalid references, non-finite values,
  division by zero, missing unit metadata, unit mismatches, dimension
  mismatches, missing required values, and analysis-status boundary violations.
- Analysis-status preservation for mechanics solved, rule-input incomplete,
  user-rule checked, user-rule failed, and human-review-required states.

## Boundary

The evaluator accepts structured expression data only. It has no parser and no
third-party expression engine because final expression grammar/library selection
remains `TBD` pending a later architecture/security decision.

This crate does not implement the required-input completeness checker, private
rule-pack lifecycle, checksum handling, public example rule packs, GUI editors,
report generation, public API transport, private storage, unit conversion
constants, or final result-envelope integration. Same-dimension arithmetic and
comparisons require matching unit references because this crate does not own
unit conversion.

## Verification

The unit tests cover successful invented expressions, dimension-compatible and
dimension-incompatible comparisons, missing and duplicate bindings, invalid
references, non-finite inputs, division by zero, unsafe construct rejection,
unsupported expression forms, status-boundary behavior, and absence of
professional/code-compliance states.
