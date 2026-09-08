# RU return

**Verdict: CHANGES_REQUIRED.**

I reviewed 100% of the frozen 2,280-line nine-path U7 diff and its required contracts. Exact hashes and focused checks are recorded in `REVIEWED_INVENTORY.sha256` and `VALIDATION.md`.

Five actionable defects are confirmed:

1. Cancel remains usable during a delayed Apply and does not invalidate App publication; the canceled batch still publishes both records and an undo checkpoint.
2. Direct reviewed Apply can publish an incomplete single-operation acceptance receipt, and its return checks do not fully bind apply-time model/operation evidence before publication.
3. A successful App commit clears “Continue from end after Apply” and its next `from` endpoint.
4. Reservation logic treats an incomplete target as valid and also permits the new node and pipe in one batch to share an ID.
5. Required provenance is prefilled as `user_entered_local_preview`, allowing publication without explicit user provenance.

The production route (12/12), inspector (9/9), selected App cases (8/8), and final-cut 1024×768 browser journey (1/1) pass. RU's private six-case suite independently reproduces the defects and confirms correct structural equality. The passing suites lack regressions for the confirmed failure modes. The historical screenshot remains pre-remediation browser evidence; native proof is pending and outside RU scope.

This technical return supports manager fan-in only as **CHANGES_REQUIRED**. Source authors were stopped throughout review. RU wrote no product or repository test source, Git, lifecycle, or sibling records. The scoped browser checks used the existing prebuilt WASM service; RU performed no Rust, WASM, or native builds and no native-app or full-harness execution.
