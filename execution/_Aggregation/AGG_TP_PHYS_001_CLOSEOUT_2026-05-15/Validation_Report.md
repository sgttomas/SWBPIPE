---
doc_id: AGG-TP-PHYS-001-VALIDATION-REPORT
doc_kind: aggregation.validation_report
status: completed
created: 2026-05-15
tranche: TP-PHYS-001
---

# TP-PHYS-001 Validation Report

## Commands Run

| Command | Result |
|---|---|
| `cargo fmt --manifest-path core/solver/frame_kernel/Cargo.toml --check` | PASS |
| `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` | PASS: 19 tests |
| `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check` | PASS |
| `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` | PASS: 12 tests |
| `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check` | PASS |
| `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` | PASS: 14 tests |
| `cargo fmt --manifest-path core/loads/user_loads/Cargo.toml --check` | PASS |
| `cargo test --manifest-path core/loads/user_loads/Cargo.toml` | PASS: 11 tests |
| `cargo fmt --manifest-path core/loads/stress_recovery/Cargo.toml --check` | PASS |
| `cargo test --manifest-path core/loads/stress_recovery/Cargo.toml` | PASS: 13 tests |
| `cargo fmt --manifest-path validation/benchmarks/mechanics/Cargo.toml --check` | PASS |
| `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml` | PASS: 10 tests |
| `cargo fmt --manifest-path validation/benchmarks/stress/Cargo.toml --check` | PASS |
| `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml` | PASS: 9 tests |
| `git diff --check` | PASS |

## Targeted Boundary Scan

Command:

```text
rg -n "certif|certified|compliance|code-compliant|ASME|allowable|SIF|flexibility|B31|professional approval|sealed|fatigue|proprietary|commercial" ...
```

Result: PASS with review notes.

The scan returned boundary statements, negative assertions, and provenance
certifications such as "not copied from protected standards" and "not an
allowable comparison." No positive professional approval, compliance,
protected-content, copied-code-formula, bundled allowable, SIF/flexibility, or
private/proprietary-data claim was identified in the TP-PHYS-001 review
surface.

## Verdict

PASS. The mechanics crates and validation benchmark crates pass the requested
format and test checks. Whitespace validation passes. Boundary scan findings
are negative/protective statements rather than prohibited claims.
