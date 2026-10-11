"""Test-only independent verifiers of producer publications.

Owner, 2026-10-10: "move to one product reader in Rust" ("All readers."). The
Rust readers in ``core/reporting/result_export`` are the only product readers.
The modules here are independent verifiers that only tests and the VP-STATIC
gate use: no product module imports them, and they share no code with the
producer (standard-library imports only; no subprocess bridge to a producer
crate). See DEL-05-02 Design ``b7-plan.md`` (Unit R).
"""
