"""Independent test verifiers of the Rust product readers' statements.

Owner, 2026-10-10: "move to one product reader in Rust" ("All readers."). The
Rust readers in ``core/reporting/result_export`` are the only product readers.
Tests only (and the VP-STATIC gate, which reads ``pressure_2.py`` by path): no
product module imports this package (`tests/test_verifier_independence.py`),
and nothing here calls a crate the producer links. See DEL-04-08 Design
``b7-plan.md`` (Unit R).
"""
