# M9 handoff state

Status: COMPLETE BOUNDED DERIVATIVE EVIDENCE / RETURNED TO HELP_HUMAN.

Accepted source basis is branch `codex/piping-physics-ui-execution-20260908` at `779dedb8670625b36af07b89fc5557470e47c50e` plus the unchanged authored fixture SHA-256 `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c`. The immutable calculation boundary is the file set and hashes in `FREEZE_MANIFEST.json`. This run record is derivative verification evidence; it is not authoritative decomposition truth or a lifecycle snapshot.

The completed physical-frame static reference gives N-130 UZ `-0.0066439267973924755 m`, S-130 UY `-52.37328198732987 N`, current normal `52.37328198732987 N`, friction action `+0.5237328198732987 N`, NL-140 UY `-297.62671801267015 N` and SH-140 UZ `+279.0085184271103 N`. Force and moment balances close to raw numerical residuals below `2.0e-9` in their respective N/N·m dimensions. Full signed displacement/support/end/station vectors are frozen in `EXPECTED_RESULTS_FROZEN.json`.

Post-freeze production comparison found:

- both frictionless variants: 157/157 rows equal after independent values are rounded to product emission precision;
- physical mixed: returned `+0.524314 N` friction versus `mu*N=0.52373217 N` from the same returned normal, residual `+0.0005818300000000054 N`;
- literal adapter: returned `+0.409403 N` friction versus `mu*N=0.40909350000000005 N`, residual `+0.0003094999999999626 N`;
- literal C-150 mapping: independently exposes an uncancelled `+658.4367607916422 N·m` connector couple.

The original fixture, pressure-free L-100 seed, connector-excluded physical control, and literal current-adapter diagnostic are separated exactly in `VARIANT_LEDGER.json`. No connector-excluded result is presented as full-fixture adequacy.

Rerun requirements are the exact commands in `REPORT.md` and `PRODUCTION_COMPARISON_EXECUTION.md`. Preserve `FREEZE_MANIFEST.json` and its hashed files byte-for-byte when comparing a later production change. A later oracle revision requires a new additive run record and new freeze; it must not rewrite this one.

Retained owner decisions are:

1. D01 current-normal production repair and re-comparison.
2. D02 C-150 topology/objectivity and the uncancelled-moment disposition.
3. SH-140 force-free/preload/reference selection.
4. Incremental friction history, committed slip and rollback semantics.
5. Explicit one-way allowed displacement side.
6. Engineering comparison thresholds and professional acceptance.
7. Canonical unit/conversion acceptance plus `PKG09-0901-PKG02-001`.

`DAG-002-E0532`, `DAG-002-E0533` and `TP-DAG-004-DEL-09-01-E001` remain unchanged. `CONSUMER_INTEGRATION_EVIDENCE.md` is suitable input to a later owner-run edge-specific revalidation; it does not satisfy those rows by itself.

One governance deviation remains visible: M9 invoked the native Rust comparison before receiving notice that F4's exclusive native/Rust slot had been released. The target was isolated at `{TASK_TEMP}/m9-cargo-target`, production crates were read-only dependencies, and no production files were changed. `PRODUCTION_COMPARISON_EXECUTION.md` is the raw path authority for the times, host resolution and mechanism. HELP_HUMAN owns the disposition; the deviation does not alter the earlier independent freeze.
