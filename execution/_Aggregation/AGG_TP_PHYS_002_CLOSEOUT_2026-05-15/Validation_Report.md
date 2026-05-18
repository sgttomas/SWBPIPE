---
doc_id: AGG-TP-PHYS-002-VALIDATION-REPORT
doc_kind: aggregation.validation_report
status: completed
created: 2026-05-15
tranche: TP-PHYS-002
---

# TP-PHYS-002 Validation Report

## Commands Run

| Command | Result |
|---|---|
| `cargo fmt --manifest-path core/solver/frame_kernel/Cargo.toml --check` | PASS |
| `cargo fmt --manifest-path core/solver/linear_supports/Cargo.toml --check` | PASS |
| `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check` | PASS |
| `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check` | PASS |
| `cargo fmt --manifest-path validation/benchmarks/mechanics/Cargo.toml --check` | PASS |
| `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` | PASS: 23 tests |
| `cargo test --manifest-path core/solver/linear_supports/Cargo.toml` | PASS: 12 tests |
| `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` | PASS: 20 tests |
| `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` | PASS: 14 tests |
| `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml` | PASS: 11 tests |
| `python3 tests/test_physical_to_analytical_transform.py` | PASS |
| `git diff --check` | PASS |

## Targeted Boundary Scan

Command:

```text
rg -n -i "certif|certified|certification|compliance|code-compliant|code compliance|ASME|allowable|SIF|flexibility|B31|professional approval|professional reliance|sealed|proprietary|commercial|protected standard|private data|vendor catalog|approval" ...
```

Result: PASS with review notes.

The scan returned boundary statements, negative assertions, and provenance
certifications such as "not copied from protected standards." No positive
professional approval, certification, code-compliance, protected-content,
bundled allowable, SIF/flexibility, private-data, or proprietary-data claim was
identified in the TP-PHYS-002 review surface.

## Verdict

PASS. The linear static engine integration crates and benchmark crate pass the
requested format, test, bridge-guard, whitespace, and boundary checks.
