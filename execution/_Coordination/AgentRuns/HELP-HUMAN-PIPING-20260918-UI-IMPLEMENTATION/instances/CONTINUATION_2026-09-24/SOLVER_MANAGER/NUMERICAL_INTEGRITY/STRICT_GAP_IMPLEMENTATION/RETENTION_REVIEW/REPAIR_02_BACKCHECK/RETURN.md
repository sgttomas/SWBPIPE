# RET-01 repair 02 backcheck

**RET-01 is closed by source backcheck. The retained-response candidate is cleared for focused execution.** No remaining blocker was found in this bounded repair. Retention tests have not yet run; this is not full NGR02, wire/persistence, public-method or Current qualification.

The adapter candidate f7600925cd44169ce6d04a03ae5c0e622a8f27aabee831377d297dce68ec51aa exactly matches the frozen file and complete delta from fd78de18. Exact-boundary kernel 6801610d is unchanged. [Source check](_run_records/SOURCE_CHECK.json), [snapshot](_run_records/FROZEN_structural_adapter.rs), [inputs](_run_records/INPUT_HASHES.json) and [origins](_run_records/ORIGINS.json) preserve the same-reviewer audit. No source/Git edit, Cargo/build/Node/native/heavy test or descendant occurred.

## Correction verified

- The prefix uses a fixed number of O(1) container-length reads and checked multiplication/addition. A failed prefix reservation returns zero charged work before any input-sized traversal.
- Once the complete prefix fits, it is retained as an accepted charge before inspecting dynamic label/error/ID lengths. Its conservative reservation covers that sizing walk even if the walk stops early.
- Dynamic bytes are accumulated separately against limit−prefix. Late overflow or budget failure returns charged=prefix and only the denied dynamic reservation in rejected; it no longer mislabels the admitted sizing work as unexecuted.
- Full admission still returns prefix+dynamic, with the same successful comparison cost as repair 01. No string-content comparison occurs before full admission.
- Summary mismatch retains that admitted charge. Exact replay receives only limit−charge; the child’s accepted work is added back and its rejected reservation is preserved. There is no reset to the original allowance.
- Checked overflow preserves the explicitly documented usize::MAX rejected-size marker. Prefix overflow has no admitted traversal; later dynamic overflow preserves the accepted prefix.

The updated C−1 and nested-error assertions now reflect those distinctions. The old 1088 limit still rejects before the larger prefix with zero charge. Long identifiers increase the reservation; sufficient-budget success and one-below-total child failure remain required controls. Exact arithmetic, original expected values, 1e-9 criterion, source ownership and replay mathematics did not change.

## Independent check and next execution

[The small control-flow model](_run_records/check_ledger.py) verifies representative short/long metadata, the old 1088 limit, C−1 late rejection, nested-error growth, checked prefix/dynamic overflow, preserved successful cost and reduced child budgets. It also checks 2,000 limit values. [Output](_run_records/LEDGER_CHECKS.json) is retained. These are source-derived reservation checks, not actual Rust fixture ledgers or timing observations.

The complete original retention review and its exact factor-two/source-tail findings remain the basis; only the accounting defect was repaired here. Run the focused exact-boundary helper tests and adapter retention tests on helper 6801610d/adapter f7600925, preserving actual short/long charges, failures and source/lock identities. Prior helper passes and the parent's disjoint default-arithmetic PASS 4 do not substitute for that retention execution.

Successful in-memory retention remains narrower than failed-attempt artifact preservation, checked wire import, persistence, full field recovery and Current eligibility. Those previously recorded limits remain open.

