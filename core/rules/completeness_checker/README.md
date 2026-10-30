# Rule-Pack Completeness Checker

This crate is the bounded implementation slice for `DEL-06-03`. It checks
declarative rule-pack required-input declarations against caller-supplied input
evidence and reports whether a user rule check is blocked by missing or
unreviewed data.

## Scope

- Required input declarations with source kind, required-for intent, dimension
  and unit requirements using the PKG-02 canonical dimension vocabulary,
  provenance requirements, and redistribution requirements.
- User input evidence records for private/user-supplied values, units,
  dimensions, provenance, redistribution status, and review status.
- Deterministic findings for missing required values, missing units,
  dimension mismatch, provenance gaps, redistribution gaps, protected-content
  suspicion, review gaps, duplicate declarations, and unexpected inputs.
- A rule-check readiness result that maps blocking findings to
  `RULE_INPUTS_INCOMPLETE` without changing mechanics solve status.

## Boundary

The crate does not evaluate formulas, parse rule packs, store private rule
data, provide code-specific values, import protected standards content, make
professional/code-compliance claims, or generate human acceptance records.
Callers own schema parsing, private storage, unit conversion, expression
evaluation, GUI presentation, report rendering, and final result-envelope
integration.

## Verification

Unit tests cover complete invented input evidence, missing values, missing
units, dimension mismatch, provenance and redistribution gaps, protected-content
suspicions, review gaps, duplicate declarations, unexpected supplied values,
reporting-only inputs, and absence of professional/code-compliance statuses.
