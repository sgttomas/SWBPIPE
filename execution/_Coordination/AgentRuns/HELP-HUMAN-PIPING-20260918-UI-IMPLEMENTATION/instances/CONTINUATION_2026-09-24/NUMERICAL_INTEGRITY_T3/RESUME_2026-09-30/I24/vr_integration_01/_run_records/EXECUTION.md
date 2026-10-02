# Execution record

Native child `/root/i24_vr_integration`, direct TASK Type2 under `/root`; no delegation. Actual instructions and exact brief origins/hashes are in ACTUAL_INSTRUCTION_ORIGINS.json and BRIEF.md. Broad supplied evidence hashes are in SUPPLIED_BASIS.json; reliance was the bounded contracts/reviews and arithmetic identified by SOURCE_MAP.md, not every body under those packet trees.

All compiler commands used the installed `+1.97.1`, RUSTUP_AUTO_INSTALL=0, CARGO_NET_OFFLINE=true, CARGO_INCREMENTAL=0, -j4, separate CARGO_TARGET_DIR from ENVIRONMENT.txt. Tests used --test-threads=2. The existing M5 guard was live. No install, registry upgrade or config change occurred.

Commands, in VR working directory unless noted:

1. `cargo +1.97.1 check --offline -j4 --all-targets` materialized seven local H/transitive lock packages. check_01.log contains a delimiter error in the new tol helper, repaired before check_02. No H dependency error was encountered.
2. `cargo +1.97.1 check --offline --locked -j4 --all-targets` — check_02.log PASS.
3. `cargo +1.97.1 test --offline --locked -j4 --test scale --test k6c_envelope -- --test-threads=2` — focused_01 and focused_02 PASS. The latter includes every caller phase/addend, not only maxima.
4. `cargo +1.97.1 test --offline --locked -j4 --lib --test scale --test k6c_envelope -- --test-threads=2` — focused_03 PASS, adding actual CLI12 and missing/overflow/context checks.
5. `cargo +1.97.1 test --offline --locked -j4 --all-targets -- --test-threads=2` — full_vr_01 and full_vr_02 PASS. Final run has 58 tests, 0 failures/ignores, including original numerical/class/control tests and feature guards.
6. `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest runner/test_vk_scale_runner.py` and later equivalent unittest discovery — runner_tests_01 (10 tests) and runner_tests_02 (12 tests) PASS.
7. `cargo +1.97.1 build --offline --locked -j4 --example vk_scale` — example_build_01 and example_build_02 PASS. Final build follows explicit u64 boundary conversion; all four final functional launches cover that boundary.
8. Exact binary argv/cwd/hashes/return codes are CLI_COMMANDS.json and CLI_COMMANDS_FINAL.json. No runner gate/performance/timing command was dispatched. Small functional normal solve was CHAIN10; the external CHAIN1000 path test is counts-only; no 10000 run.
9. `rustfmt +1.97.1 --edition 2021` on owned Rust files only; final scoped `git diff --check` passes. Cargo.lock registry blocks were compared byte-for-byte to base; additions are local only.

One early status/head discovery and a later scoped diff lacked GIT_OPTIONAL_LOCKS=0; the subsequent Git reads explicitly set it. No explicit Git index/stage/commit/push command was issued. Any incidental Git cache refresh from those read commands was not requested or relied upon. No unrelated state was reverted or staged. H changes belong to concurrent I21 and were only read as the local dependency.

Two evidence-sealing helper attempts failed before VERIFICATION.json was written: system Python lacks tomllib, so lock blocks were compared directly; an initially assumed suite count was wrong, so the actual 13 test-result groups / 58 tests were read and recorded. Neither changed maintained source or test assertions. Full raw Rust/CLI/Python runner outputs remain intact.

The tests are author validation, not independent implementation review. Original fixed raw-input/finite-numerator/library/request/compiler/profile premises remain conditional; arbitrary escaped/non-u64 JSON scratch and other unsupported descriptors return named errors, never a partial max. Current executable/admission/measurement qualification remains with ROOT.
