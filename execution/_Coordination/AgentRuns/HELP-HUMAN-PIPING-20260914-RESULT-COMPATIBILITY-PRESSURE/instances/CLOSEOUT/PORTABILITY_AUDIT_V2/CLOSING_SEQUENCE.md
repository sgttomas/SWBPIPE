# Minimal portable closing sequence

1. Let compatibility qualification and native consumer preparation finish; release the serialized lease and stop all writers.
2. Reconcile the writer `STATUS.json` with its return and update Root’s dispatch ledger through the owning workflow.
3. Freeze the final source/evidence inventory. Re-run the existing path-anchor validator and classifier; do not use a replacement scanner.
4. Create the portable successor controls listed in `FUTURE_FREEZE_REVISITS.md`. Preserve the original host-specific controls and raw records byte-for-byte as historical run evidence.
5. Recompute exact hashes for every draft candidate. Drop or revise any entry whose bytes, role, completion status, or path changed.
6. Root reviews and adopts the additive policy delta. Append entries only; preserve prior policy entries.
7. Re-run path-anchor validation. The selected run should contribute zero unacknowledged control or active-unclassified findings.
8. Continue the prepared closeout receipt, final verification/CI, closure snapshot, and Git packaging sequence only after the portability result is clean.
