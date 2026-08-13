# WORKING_ITEMS Run — DEL-10-05 Mechanics Runner Binding

RunID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

Package / deliverable: `PKG-10` / `DEL-10-05`

Authority: owner-adopted
`CB-2026-08-11-DEL1005-MECHANICS-RUNNER-BINDING-001`, candidate SHA-256
`c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`,
pinned base `f1e311fb7ab1c2a0800b1d32c59445368428dee9`.

Dependency: accepted DEL-09-01 SHA-256
`18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`,
Git blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`.

## Accepted output

- `core/runner/headless/src/benchmark_binding.rs`
- SHA-256 `4a45a0889391046fe6ab887409c791a2148bc30205478138a142af07fd4f1e6f`
- Git blob `455b9e9dee1a6a8154f65a1b5218dd7e09b6444f`

The original explicit 11-case evaluator paths remain intact. The formerly
blocked fallback consumes the suite-owned observation and comparison
accessors. No conditional binary/Python file, schema, runner tolerance,
expected value, fixture formula, or mechanics C-B behavior was added.

## Validation and fan-in

- Fresh N2 remediation author and non-repairing verifier: PASS.
- N3 independent child return SHA-256
  `e7dfee006d050636e38e0e5def29f1acdda487902ea5f3928dd6925fde6de19b`:
  `COMMIT_SAFE`.
- Runner tests 38/38 library, 1/1 preview, 15/15 final binary; mechanics 41/41.
- Whole suite 25/25 matched, 0 mismatched, 0 blocked, 206/206 finite
  populated comparisons; new exact 14/115; original complete objects exact
  11/91.
- All six required fail-closed behaviors PASS.
- Stress unchanged; nonlinear 5/5; C-B exact and nonlinear-only.
- Formatting, diff, exact source containment, staged-zero, ignored-zero: PASS.

The initial ignored lockfile side effect, exact owner-authorized one-file
cleanup, fresh remediation, and final verification remain preserved in the
managed run records.

## State and handoff

This is bounded implementation evidence only. Lifecycle remains
`IN_PROGRESS`. The public result-comparison number, validation-manual
promotion, Task Management, release/publication, and reliance decisions remain
open or outside scope. Standard claim fence applies (F-PIP-2; claims taxonomy
per DEC-081).

The clean-commit DEC-025 sweep is derivative evidence and is not yet produced.
Rerun if accepted mechanics or runner bytes, frozen output, stress/nonlinear
sources, C-B bytes, Cargo resolution, or cleanliness changes. Next owner:
CHANGE for the exact local candidate commit, then DEC-025; push/PR remain behind
a publication token and merge remains the owner's act.
