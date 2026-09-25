# PR905 initial CI repairs

The first hosted run at `e65001ad50072d3399bf204da02055b0f360353a` exposed two integration-record issues before full qualification. No solver accuracy pass is inferred from their repair.

The governance conflict check found64 markers in four retained pre-repair source captures. Their exact bytes remain at immutable pushed Git addresses, with SHA-256/size/blob identities in [the archive index](../SOURCE_BLOCK_FANIN/READER_RESOLUTION/PREIMAGE_ARCHIVE.md). Maintained resolved source is unchanged. A narrow independent [backcheck](_run_records/preimage_review/RETURN.md) is clear; the guard and its criterion are unchanged.

The numerical job stopped at locked dependency fetch for report_package. Checking all39 current crate graphs found four stale leaf lockfiles: report_package and the mechanics, numerical_integrity and stress benchmarks. The repair records required local dependency edges and their transitive closure. Every existing package version and checksum is preserved; no application/Tauri lock, manifest or solver source is changed. [Exact dependency delta](_run_records/lock-check/REPAIR.json) identifies added packages.

All39 offline locked fetch checks now pass. The four affected Cargo test commands exit0:83 tests pass across report_package/mechanics/stress; numerical_integrity builds with zero registered Rust tests, so no numerical benchmark execution is claimed. [Command outcomes](_run_records/lock-check/TEST_RESULTS.json) and raw logs retain that distinction. INITIAL_RESULTS.json preserves the first local four-failure command summary; numbered fetch logs are the repaired rerun, not the initial failure logs. The original hosted failure has its own unchanged numerical artifact and job log.

Independent lock-impact review and actual next-head hosted CI/clean DEC-025 remain pending. This repair does not close another original finding, change analytical references, repeat native witnesses or claim engineering acceptance.
