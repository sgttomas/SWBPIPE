# Proposed commands — NOT EXECUTED

Placeholders bind only after ROOT grants a runtime block. ARCHIVE must contain complete frozen core+VR from81c038. SEEDED_TARGET is exclusive to the seeded source/profile; MECHANICAL_TARGET is separate. ARTIFACT keys resolve only from successful new Cargo JSON. RAW_LOGS stays inside the granted owned raw-record location.

Unset only the four preserved per-command compiler override variables; no user/global configuration is changed. Keep existing M5 guard live. No --write is allowed.

## B1_ADAPTER

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE cargo test --offline --locked -j 4 --manifest-path '<ARCHIVE>/projects/chirality-piping/validation/benchmarks/numerical_robustness/Cargo.toml' --target-dir '<SEEDED_TARGET>' --features seeded-faults --test adapter --no-run --message-format=json
```

## B2_LANE_PARITY

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE cargo test --offline --locked -j 4 --manifest-path '<ARCHIVE>/projects/chirality-piping/validation/benchmarks/numerical_robustness/Cargo.toml' --target-dir '<SEEDED_TARGET>' --features seeded-faults --test lane --test parity --no-run --message-format=json
```

## B3_RECORDS

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE cargo build --offline --locked -j 4 --release --manifest-path '<ARCHIVE>/projects/chirality-piping/validation/benchmarks/numerical_robustness/Cargo.toml' --target-dir '<SEEDED_TARGET>' --features seeded-faults --example vk_records --message-format=json
```

## VK-F01-records-RF-CHAIN

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_f01_diagnostic_01/D02_F01/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-CHAIN --show=RF-CHAIN-T-n03-r1e-04 --show=RF-CHAIN-T-n03-r1e-06 --show=RF-CHAIN-T-n03-r1e-08 --show=RF-CHAIN-T-n03-r1e-10 --show=RF-CHAIN-T-n03-r1e-12 --show=RF-CHAIN-T-n05-r1e-04 --show=RF-CHAIN-T-n05-r1e-06 --show=RF-CHAIN-T-n05-r1e-08 --show=RF-CHAIN-T-n05-r1e-10 --show=RF-CHAIN-T-n05-r1e-12 --show=RF-CHAIN-T-n10-r1e-04 --show=RF-CHAIN-T-n10-r1e-06 --show=RF-CHAIN-T-n10-r1e-08 --show=RF-CHAIN-T-n10-r1e-10 --show=RF-CHAIN-T-n10-r1e-12 --show=RF-CHAIN-A-n03-r1e-04 --show=RF-CHAIN-A-n03-r1e-06 --show=RF-CHAIN-A-n03-r1e-08 --show=RF-CHAIN-A-n03-r1e-10 --show=RF-CHAIN-A-n03-r1e-12 --show=RF-CHAIN-A-n05-r1e-04 --show=RF-CHAIN-A-n05-r1e-06 --show=RF-CHAIN-A-n05-r1e-08 --show=RF-CHAIN-A-n05-r1e-10 --show=RF-CHAIN-A-n05-r1e-12 --show=RF-CHAIN-A-n10-r1e-04 --show=RF-CHAIN-A-n10-r1e-06 --show=RF-CHAIN-A-n10-r1e-08 --show=RF-CHAIN-A-n10-r1e-10 --show=RF-CHAIN-A-n10-r1e-12 > '<RAW_LOGS>/VK-F01-records-RF-CHAIN/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F01-records-RF-CHAIN/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F01 '<ARTIFACT:vk_records-release-seeded>' RF-CHAIN --show=RF-CHAIN-T-n03-r1e-04 --show=RF-CHAIN-T-n03-r1e-06 --show=RF-CHAIN-T-n03-r1e-08 --show=RF-CHAIN-T-n03-r1e-10 --show=RF-CHAIN-T-n03-r1e-12 --show=RF-CHAIN-T-n05-r1e-04 --show=RF-CHAIN-T-n05-r1e-06 --show=RF-CHAIN-T-n05-r1e-08 --show=RF-CHAIN-T-n05-r1e-10 --show=RF-CHAIN-T-n05-r1e-12 --show=RF-CHAIN-T-n10-r1e-04 --show=RF-CHAIN-T-n10-r1e-06 --show=RF-CHAIN-T-n10-r1e-08 --show=RF-CHAIN-T-n10-r1e-10 --show=RF-CHAIN-T-n10-r1e-12 --show=RF-CHAIN-A-n03-r1e-04 --show=RF-CHAIN-A-n03-r1e-06 --show=RF-CHAIN-A-n03-r1e-08 --show=RF-CHAIN-A-n03-r1e-10 --show=RF-CHAIN-A-n03-r1e-12 --show=RF-CHAIN-A-n05-r1e-04 --show=RF-CHAIN-A-n05-r1e-06 --show=RF-CHAIN-A-n05-r1e-08 --show=RF-CHAIN-A-n05-r1e-10 --show=RF-CHAIN-A-n05-r1e-12 --show=RF-CHAIN-A-n10-r1e-04 --show=RF-CHAIN-A-n10-r1e-06 --show=RF-CHAIN-A-n10-r1e-08 --show=RF-CHAIN-A-n10-r1e-10 --show=RF-CHAIN-A-n10-r1e-12 > '<RAW_LOGS>/VK-F01-records-RF-CHAIN/fault.stdout' 2> '<RAW_LOGS>/VK-F01-records-RF-CHAIN/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-CHAIN --show=RF-CHAIN-T-n03-r1e-04 --show=RF-CHAIN-T-n03-r1e-06 --show=RF-CHAIN-T-n03-r1e-08 --show=RF-CHAIN-T-n03-r1e-10 --show=RF-CHAIN-T-n03-r1e-12 --show=RF-CHAIN-T-n05-r1e-04 --show=RF-CHAIN-T-n05-r1e-06 --show=RF-CHAIN-T-n05-r1e-08 --show=RF-CHAIN-T-n05-r1e-10 --show=RF-CHAIN-T-n05-r1e-12 --show=RF-CHAIN-T-n10-r1e-04 --show=RF-CHAIN-T-n10-r1e-06 --show=RF-CHAIN-T-n10-r1e-08 --show=RF-CHAIN-T-n10-r1e-10 --show=RF-CHAIN-T-n10-r1e-12 --show=RF-CHAIN-A-n03-r1e-04 --show=RF-CHAIN-A-n03-r1e-06 --show=RF-CHAIN-A-n03-r1e-08 --show=RF-CHAIN-A-n03-r1e-10 --show=RF-CHAIN-A-n03-r1e-12 --show=RF-CHAIN-A-n05-r1e-04 --show=RF-CHAIN-A-n05-r1e-06 --show=RF-CHAIN-A-n05-r1e-08 --show=RF-CHAIN-A-n05-r1e-10 --show=RF-CHAIN-A-n05-r1e-12 --show=RF-CHAIN-A-n10-r1e-04 --show=RF-CHAIN-A-n10-r1e-06 --show=RF-CHAIN-A-n10-r1e-08 --show=RF-CHAIN-A-n10-r1e-10 --show=RF-CHAIN-A-n10-r1e-12 > '<RAW_LOGS>/VK-F01-records-RF-CHAIN/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F01-records-RF-CHAIN/NONE_after.stderr'
```

## VK-F02-rf_chain

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_02/P11/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_chain --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F02-rf_chain/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F02-rf_chain/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F02 '<ARTIFACT:lane-debug-seeded>' --exact rf_chain --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F02-rf_chain/fault.stdout' 2> '<RAW_LOGS>/VK-F02-rf_chain/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_chain --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F02-rf_chain/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F02-rf_chain/NONE_after.stderr'
```

## VK-F03-records-RF-CHAIN

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_f03_diagnostic_01/D02_F03/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-CHAIN RF-SKEW --show=RF-CHAIN-T-n03-r1e-04 --show=RF-CHAIN-T-n03-r1e-06 --show=RF-CHAIN-T-n03-r1e-08 --show=RF-CHAIN-T-n03-r1e-10 --show=RF-CHAIN-T-n03-r1e-12 --show=RF-CHAIN-T-n05-r1e-04 --show=RF-CHAIN-T-n05-r1e-06 --show=RF-CHAIN-T-n05-r1e-08 --show=RF-CHAIN-T-n05-r1e-10 --show=RF-CHAIN-T-n05-r1e-12 --show=RF-CHAIN-T-n10-r1e-04 --show=RF-CHAIN-T-n10-r1e-06 --show=RF-CHAIN-T-n10-r1e-08 --show=RF-CHAIN-T-n10-r1e-10 --show=RF-CHAIN-T-n10-r1e-12 --show=RF-CHAIN-A-n03-r1e-04 --show=RF-CHAIN-A-n03-r1e-06 --show=RF-CHAIN-A-n03-r1e-08 --show=RF-CHAIN-A-n03-r1e-10 --show=RF-CHAIN-A-n03-r1e-12 --show=RF-CHAIN-A-n05-r1e-04 --show=RF-CHAIN-A-n05-r1e-06 --show=RF-CHAIN-A-n05-r1e-08 --show=RF-CHAIN-A-n05-r1e-10 --show=RF-CHAIN-A-n05-r1e-12 --show=RF-CHAIN-A-n10-r1e-04 --show=RF-CHAIN-A-n10-r1e-06 --show=RF-CHAIN-A-n10-r1e-08 --show=RF-CHAIN-A-n10-r1e-10 --show=RF-CHAIN-A-n10-r1e-12 --show=RF-SKEW-T-PIN-AX-345-r1e-04 --show=RF-SKEW-T-PIN-AX-345-r1e-08 --show=RF-SKEW-T-PIN-AX-345-r1e-12 --show=RF-SKEW-T-PIN-AX-122-r1e-04 --show=RF-SKEW-T-PIN-AX-122-r1e-08 --show=RF-SKEW-T-PIN-AX-122-r1e-12 --show=RF-SKEW-T-PIN-OFF-345-r1e-04 --show=RF-SKEW-T-PIN-OFF-345-r1e-08 --show=RF-SKEW-T-PIN-OFF-345-r1e-12 --show=RF-SKEW-T-PIN-OFF-122-r1e-04 --show=RF-SKEW-T-PIN-OFF-122-r1e-08 --show=RF-SKEW-T-PIN-OFF-122-r1e-12 --show=RF-SKEW-T-CANT-AX-345-r1e-04 --show=RF-SKEW-T-CANT-AX-345-r1e-08 --show=RF-SKEW-T-CANT-AX-345-r1e-12 --show=RF-SKEW-T-CANT-AX-122-r1e-04 --show=RF-SKEW-T-CANT-AX-122-r1e-08 --show=RF-SKEW-T-CANT-AX-122-r1e-12 --show=RF-SKEW-T-CANT-OFF-345-r1e-04 --show=RF-SKEW-T-CANT-OFF-345-r1e-08 --show=RF-SKEW-T-CANT-OFF-345-r1e-12 --show=RF-SKEW-T-CANT-OFF-122-r1e-04 --show=RF-SKEW-T-CANT-OFF-122-r1e-08 --show=RF-SKEW-T-CANT-OFF-122-r1e-12 --show=RF-SKEW-A-CANT-AX-345-r1e-04 --show=RF-SKEW-A-CANT-AX-345-r1e-08 --show=RF-SKEW-A-CANT-AX-345-r1e-12 --show=RF-SKEW-A-CANT-AX-122-r1e-04 --show=RF-SKEW-A-CANT-AX-122-r1e-08 --show=RF-SKEW-A-CANT-AX-122-r1e-12 --show=RF-SKEW-A-CANT-OFF-345-r1e-04 --show=RF-SKEW-A-CANT-OFF-345-r1e-08 --show=RF-SKEW-A-CANT-OFF-345-r1e-12 --show=RF-SKEW-A-CANT-OFF-122-r1e-04 --show=RF-SKEW-A-CANT-OFF-122-r1e-08 --show=RF-SKEW-A-CANT-OFF-122-r1e-12 > '<RAW_LOGS>/VK-F03-records-RF-CHAIN/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F03-records-RF-CHAIN/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F03 '<ARTIFACT:vk_records-release-seeded>' RF-CHAIN RF-SKEW --show=RF-CHAIN-T-n03-r1e-04 --show=RF-CHAIN-T-n03-r1e-06 --show=RF-CHAIN-T-n03-r1e-08 --show=RF-CHAIN-T-n03-r1e-10 --show=RF-CHAIN-T-n03-r1e-12 --show=RF-CHAIN-T-n05-r1e-04 --show=RF-CHAIN-T-n05-r1e-06 --show=RF-CHAIN-T-n05-r1e-08 --show=RF-CHAIN-T-n05-r1e-10 --show=RF-CHAIN-T-n05-r1e-12 --show=RF-CHAIN-T-n10-r1e-04 --show=RF-CHAIN-T-n10-r1e-06 --show=RF-CHAIN-T-n10-r1e-08 --show=RF-CHAIN-T-n10-r1e-10 --show=RF-CHAIN-T-n10-r1e-12 --show=RF-CHAIN-A-n03-r1e-04 --show=RF-CHAIN-A-n03-r1e-06 --show=RF-CHAIN-A-n03-r1e-08 --show=RF-CHAIN-A-n03-r1e-10 --show=RF-CHAIN-A-n03-r1e-12 --show=RF-CHAIN-A-n05-r1e-04 --show=RF-CHAIN-A-n05-r1e-06 --show=RF-CHAIN-A-n05-r1e-08 --show=RF-CHAIN-A-n05-r1e-10 --show=RF-CHAIN-A-n05-r1e-12 --show=RF-CHAIN-A-n10-r1e-04 --show=RF-CHAIN-A-n10-r1e-06 --show=RF-CHAIN-A-n10-r1e-08 --show=RF-CHAIN-A-n10-r1e-10 --show=RF-CHAIN-A-n10-r1e-12 --show=RF-SKEW-T-PIN-AX-345-r1e-04 --show=RF-SKEW-T-PIN-AX-345-r1e-08 --show=RF-SKEW-T-PIN-AX-345-r1e-12 --show=RF-SKEW-T-PIN-AX-122-r1e-04 --show=RF-SKEW-T-PIN-AX-122-r1e-08 --show=RF-SKEW-T-PIN-AX-122-r1e-12 --show=RF-SKEW-T-PIN-OFF-345-r1e-04 --show=RF-SKEW-T-PIN-OFF-345-r1e-08 --show=RF-SKEW-T-PIN-OFF-345-r1e-12 --show=RF-SKEW-T-PIN-OFF-122-r1e-04 --show=RF-SKEW-T-PIN-OFF-122-r1e-08 --show=RF-SKEW-T-PIN-OFF-122-r1e-12 --show=RF-SKEW-T-CANT-AX-345-r1e-04 --show=RF-SKEW-T-CANT-AX-345-r1e-08 --show=RF-SKEW-T-CANT-AX-345-r1e-12 --show=RF-SKEW-T-CANT-AX-122-r1e-04 --show=RF-SKEW-T-CANT-AX-122-r1e-08 --show=RF-SKEW-T-CANT-AX-122-r1e-12 --show=RF-SKEW-T-CANT-OFF-345-r1e-04 --show=RF-SKEW-T-CANT-OFF-345-r1e-08 --show=RF-SKEW-T-CANT-OFF-345-r1e-12 --show=RF-SKEW-T-CANT-OFF-122-r1e-04 --show=RF-SKEW-T-CANT-OFF-122-r1e-08 --show=RF-SKEW-T-CANT-OFF-122-r1e-12 --show=RF-SKEW-A-CANT-AX-345-r1e-04 --show=RF-SKEW-A-CANT-AX-345-r1e-08 --show=RF-SKEW-A-CANT-AX-345-r1e-12 --show=RF-SKEW-A-CANT-AX-122-r1e-04 --show=RF-SKEW-A-CANT-AX-122-r1e-08 --show=RF-SKEW-A-CANT-AX-122-r1e-12 --show=RF-SKEW-A-CANT-OFF-345-r1e-04 --show=RF-SKEW-A-CANT-OFF-345-r1e-08 --show=RF-SKEW-A-CANT-OFF-345-r1e-12 --show=RF-SKEW-A-CANT-OFF-122-r1e-04 --show=RF-SKEW-A-CANT-OFF-122-r1e-08 --show=RF-SKEW-A-CANT-OFF-122-r1e-12 > '<RAW_LOGS>/VK-F03-records-RF-CHAIN/fault.stdout' 2> '<RAW_LOGS>/VK-F03-records-RF-CHAIN/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-CHAIN RF-SKEW --show=RF-CHAIN-T-n03-r1e-04 --show=RF-CHAIN-T-n03-r1e-06 --show=RF-CHAIN-T-n03-r1e-08 --show=RF-CHAIN-T-n03-r1e-10 --show=RF-CHAIN-T-n03-r1e-12 --show=RF-CHAIN-T-n05-r1e-04 --show=RF-CHAIN-T-n05-r1e-06 --show=RF-CHAIN-T-n05-r1e-08 --show=RF-CHAIN-T-n05-r1e-10 --show=RF-CHAIN-T-n05-r1e-12 --show=RF-CHAIN-T-n10-r1e-04 --show=RF-CHAIN-T-n10-r1e-06 --show=RF-CHAIN-T-n10-r1e-08 --show=RF-CHAIN-T-n10-r1e-10 --show=RF-CHAIN-T-n10-r1e-12 --show=RF-CHAIN-A-n03-r1e-04 --show=RF-CHAIN-A-n03-r1e-06 --show=RF-CHAIN-A-n03-r1e-08 --show=RF-CHAIN-A-n03-r1e-10 --show=RF-CHAIN-A-n03-r1e-12 --show=RF-CHAIN-A-n05-r1e-04 --show=RF-CHAIN-A-n05-r1e-06 --show=RF-CHAIN-A-n05-r1e-08 --show=RF-CHAIN-A-n05-r1e-10 --show=RF-CHAIN-A-n05-r1e-12 --show=RF-CHAIN-A-n10-r1e-04 --show=RF-CHAIN-A-n10-r1e-06 --show=RF-CHAIN-A-n10-r1e-08 --show=RF-CHAIN-A-n10-r1e-10 --show=RF-CHAIN-A-n10-r1e-12 --show=RF-SKEW-T-PIN-AX-345-r1e-04 --show=RF-SKEW-T-PIN-AX-345-r1e-08 --show=RF-SKEW-T-PIN-AX-345-r1e-12 --show=RF-SKEW-T-PIN-AX-122-r1e-04 --show=RF-SKEW-T-PIN-AX-122-r1e-08 --show=RF-SKEW-T-PIN-AX-122-r1e-12 --show=RF-SKEW-T-PIN-OFF-345-r1e-04 --show=RF-SKEW-T-PIN-OFF-345-r1e-08 --show=RF-SKEW-T-PIN-OFF-345-r1e-12 --show=RF-SKEW-T-PIN-OFF-122-r1e-04 --show=RF-SKEW-T-PIN-OFF-122-r1e-08 --show=RF-SKEW-T-PIN-OFF-122-r1e-12 --show=RF-SKEW-T-CANT-AX-345-r1e-04 --show=RF-SKEW-T-CANT-AX-345-r1e-08 --show=RF-SKEW-T-CANT-AX-345-r1e-12 --show=RF-SKEW-T-CANT-AX-122-r1e-04 --show=RF-SKEW-T-CANT-AX-122-r1e-08 --show=RF-SKEW-T-CANT-AX-122-r1e-12 --show=RF-SKEW-T-CANT-OFF-345-r1e-04 --show=RF-SKEW-T-CANT-OFF-345-r1e-08 --show=RF-SKEW-T-CANT-OFF-345-r1e-12 --show=RF-SKEW-T-CANT-OFF-122-r1e-04 --show=RF-SKEW-T-CANT-OFF-122-r1e-08 --show=RF-SKEW-T-CANT-OFF-122-r1e-12 --show=RF-SKEW-A-CANT-AX-345-r1e-04 --show=RF-SKEW-A-CANT-AX-345-r1e-08 --show=RF-SKEW-A-CANT-AX-345-r1e-12 --show=RF-SKEW-A-CANT-AX-122-r1e-04 --show=RF-SKEW-A-CANT-AX-122-r1e-08 --show=RF-SKEW-A-CANT-AX-122-r1e-12 --show=RF-SKEW-A-CANT-OFF-345-r1e-04 --show=RF-SKEW-A-CANT-OFF-345-r1e-08 --show=RF-SKEW-A-CANT-OFF-345-r1e-12 --show=RF-SKEW-A-CANT-OFF-122-r1e-04 --show=RF-SKEW-A-CANT-OFF-122-r1e-08 --show=RF-SKEW-A-CANT-OFF-122-r1e-12 > '<RAW_LOGS>/VK-F03-records-RF-CHAIN/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F03-records-RF-CHAIN/NONE_after.stderr'
```

## VK-F04-rf_invariance

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_03/P19/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_invariance --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F04-rf_invariance/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F04-rf_invariance/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F04 '<ARTIFACT:lane-debug-seeded>' --exact rf_invariance --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F04-rf_invariance/fault.stdout' 2> '<RAW_LOGS>/VK-F04-rf_invariance/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_invariance --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F04-rf_invariance/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F04-rf_invariance/NONE_after.stderr'
```

## VK-F04-rf_skew

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_03/P17/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_skew --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F04-rf_skew/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F04-rf_skew/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F04 '<ARTIFACT:lane-debug-seeded>' --exact rf_skew --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F04-rf_skew/fault.stdout' 2> '<RAW_LOGS>/VK-F04-rf_skew/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_skew --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F04-rf_skew/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F04-rf_skew/NONE_after.stderr'
```

## VK-F05-records-RF-RANGE

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_09/P40/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-RANGE --show=RF-RANGE-THIN-A --show=RF-RANGE-THIN-B > '<RAW_LOGS>/VK-F05-records-RF-RANGE/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F05-records-RF-RANGE/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F05 '<ARTIFACT:vk_records-release-seeded>' RF-RANGE --show=RF-RANGE-THIN-A --show=RF-RANGE-THIN-B > '<RAW_LOGS>/VK-F05-records-RF-RANGE/fault.stdout' 2> '<RAW_LOGS>/VK-F05-records-RF-RANGE/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-RANGE --show=RF-RANGE-THIN-A --show=RF-RANGE-THIN-B > '<RAW_LOGS>/VK-F05-records-RF-RANGE/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F05-records-RF-RANGE/NONE_after.stderr'
```

## VK-F06-records-RF-RANGE

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_09/P43/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-RANGE --show=RF-RANGE-THIN-A --show=RF-RANGE-THIN-B > '<RAW_LOGS>/VK-F06-records-RF-RANGE/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F06-records-RF-RANGE/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F06 '<ARTIFACT:vk_records-release-seeded>' RF-RANGE --show=RF-RANGE-THIN-A --show=RF-RANGE-THIN-B > '<RAW_LOGS>/VK-F06-records-RF-RANGE/fault.stdout' 2> '<RAW_LOGS>/VK-F06-records-RF-RANGE/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-RANGE --show=RF-RANGE-THIN-A --show=RF-RANGE-THIN-B > '<RAW_LOGS>/VK-F06-records-RF-RANGE/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F06-records-RF-RANGE/NONE_after.stderr'
```

## VK-F07-records-RF-MECH

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_09/P46/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-MECH --show=RF-MECH-LINE122-TORQUE --show=RF-MECH-LINE122-PERP --show=RF-MECH-LINE122-UNLOADED --show=RF-MECH-LINE345-RZ --show=RF-MECH-K0 --show=RF-MECH-DISC-CHAIN100 --show=RF-MECH-DISC-CHAIN100-SPRING --show=RF-MECH-LINE-IN-CHAIN1000 > '<RAW_LOGS>/VK-F07-records-RF-MECH/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F07-records-RF-MECH/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F07 '<ARTIFACT:vk_records-release-seeded>' RF-MECH --show=RF-MECH-LINE122-TORQUE --show=RF-MECH-LINE122-PERP --show=RF-MECH-LINE122-UNLOADED --show=RF-MECH-LINE345-RZ --show=RF-MECH-K0 --show=RF-MECH-DISC-CHAIN100 --show=RF-MECH-DISC-CHAIN100-SPRING --show=RF-MECH-LINE-IN-CHAIN1000 > '<RAW_LOGS>/VK-F07-records-RF-MECH/fault.stdout' 2> '<RAW_LOGS>/VK-F07-records-RF-MECH/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-MECH --show=RF-MECH-LINE122-TORQUE --show=RF-MECH-LINE122-PERP --show=RF-MECH-LINE122-UNLOADED --show=RF-MECH-LINE345-RZ --show=RF-MECH-K0 --show=RF-MECH-DISC-CHAIN100 --show=RF-MECH-DISC-CHAIN100-SPRING --show=RF-MECH-LINE-IN-CHAIN1000 > '<RAW_LOGS>/VK-F07-records-RF-MECH/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F07-records-RF-MECH/NONE_after.stderr'
```

## VK-F08-rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_04/P21/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:parity-debug-seeded>' --exact rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F08-rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F08-rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F08 '<ARTIFACT:parity-debug-seeded>' --exact rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F08-rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis/fault.stdout' 2> '<RAW_LOGS>/VK-F08-rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:parity-debug-seeded>' --exact rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F08-rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F08-rf_large_at_10_and_100_members_has_bitwise_k_class_parity_and_the_dec053_basis/NONE_after.stderr'
```

## VK-F10-permuting_every_list_changes_no_canonical_byte

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_04/P23/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:adapter-debug-seeded>' --exact permuting_every_list_changes_no_canonical_byte --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F10-permuting_every_list_changes_no_canonical_byte/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F10-permuting_every_list_changes_no_canonical_byte/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F10 '<ARTIFACT:adapter-debug-seeded>' --exact permuting_every_list_changes_no_canonical_byte --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F10-permuting_every_list_changes_no_canonical_byte/fault.stdout' 2> '<RAW_LOGS>/VK-F10-permuting_every_list_changes_no_canonical_byte/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:adapter-debug-seeded>' --exact permuting_every_list_changes_no_canonical_byte --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F10-permuting_every_list_changes_no_canonical_byte/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F10-permuting_every_list_changes_no_canonical_byte/NONE_after.stderr'
```

## VK-F13-rf_cancel

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_04/P25/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_cancel --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F13-rf_cancel/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F13-rf_cancel/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F13 '<ARTIFACT:lane-debug-seeded>' --exact rf_cancel --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F13-rf_cancel/fault.stdout' 2> '<RAW_LOGS>/VK-F13-rf_cancel/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_cancel --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F13-rf_cancel/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F13-rf_cancel/NONE_after.stderr'
```

## VK-F17-rf_weak

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_04/P27/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_weak --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F17-rf_weak/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F17-rf_weak/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F17 '<ARTIFACT:lane-debug-seeded>' --exact rf_weak --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F17-rf_weak/fault.stdout' 2> '<RAW_LOGS>/VK-F17-rf_weak/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_weak --test-threads=2 --nocapture > '<RAW_LOGS>/VK-F17-rf_weak/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F17-rf_weak/NONE_after.stderr'
```

## VK-F17-records-RF-SKEW

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_f17_diagnostic_01/D02_F17/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-SKEW --show=RF-SKEW-T-PIN-AX-345-r1e-04 --show=RF-SKEW-T-PIN-AX-345-r1e-08 --show=RF-SKEW-T-PIN-AX-345-r1e-12 --show=RF-SKEW-T-PIN-AX-122-r1e-04 --show=RF-SKEW-T-PIN-AX-122-r1e-08 --show=RF-SKEW-T-PIN-AX-122-r1e-12 --show=RF-SKEW-T-PIN-OFF-345-r1e-04 --show=RF-SKEW-T-PIN-OFF-345-r1e-08 --show=RF-SKEW-T-PIN-OFF-345-r1e-12 --show=RF-SKEW-T-PIN-OFF-122-r1e-04 --show=RF-SKEW-T-PIN-OFF-122-r1e-08 --show=RF-SKEW-T-PIN-OFF-122-r1e-12 --show=RF-SKEW-T-CANT-AX-345-r1e-04 --show=RF-SKEW-T-CANT-AX-345-r1e-08 --show=RF-SKEW-T-CANT-AX-345-r1e-12 --show=RF-SKEW-T-CANT-AX-122-r1e-04 --show=RF-SKEW-T-CANT-AX-122-r1e-08 --show=RF-SKEW-T-CANT-AX-122-r1e-12 --show=RF-SKEW-T-CANT-OFF-345-r1e-04 --show=RF-SKEW-T-CANT-OFF-345-r1e-08 --show=RF-SKEW-T-CANT-OFF-345-r1e-12 --show=RF-SKEW-T-CANT-OFF-122-r1e-04 --show=RF-SKEW-T-CANT-OFF-122-r1e-08 --show=RF-SKEW-T-CANT-OFF-122-r1e-12 --show=RF-SKEW-A-CANT-AX-345-r1e-04 --show=RF-SKEW-A-CANT-AX-345-r1e-08 --show=RF-SKEW-A-CANT-AX-345-r1e-12 --show=RF-SKEW-A-CANT-AX-122-r1e-04 --show=RF-SKEW-A-CANT-AX-122-r1e-08 --show=RF-SKEW-A-CANT-AX-122-r1e-12 --show=RF-SKEW-A-CANT-OFF-345-r1e-04 --show=RF-SKEW-A-CANT-OFF-345-r1e-08 --show=RF-SKEW-A-CANT-OFF-345-r1e-12 --show=RF-SKEW-A-CANT-OFF-122-r1e-04 --show=RF-SKEW-A-CANT-OFF-122-r1e-08 --show=RF-SKEW-A-CANT-OFF-122-r1e-12 > '<RAW_LOGS>/VK-F17-records-RF-SKEW/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F17-records-RF-SKEW/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F17 '<ARTIFACT:vk_records-release-seeded>' RF-SKEW --show=RF-SKEW-T-PIN-AX-345-r1e-04 --show=RF-SKEW-T-PIN-AX-345-r1e-08 --show=RF-SKEW-T-PIN-AX-345-r1e-12 --show=RF-SKEW-T-PIN-AX-122-r1e-04 --show=RF-SKEW-T-PIN-AX-122-r1e-08 --show=RF-SKEW-T-PIN-AX-122-r1e-12 --show=RF-SKEW-T-PIN-OFF-345-r1e-04 --show=RF-SKEW-T-PIN-OFF-345-r1e-08 --show=RF-SKEW-T-PIN-OFF-345-r1e-12 --show=RF-SKEW-T-PIN-OFF-122-r1e-04 --show=RF-SKEW-T-PIN-OFF-122-r1e-08 --show=RF-SKEW-T-PIN-OFF-122-r1e-12 --show=RF-SKEW-T-CANT-AX-345-r1e-04 --show=RF-SKEW-T-CANT-AX-345-r1e-08 --show=RF-SKEW-T-CANT-AX-345-r1e-12 --show=RF-SKEW-T-CANT-AX-122-r1e-04 --show=RF-SKEW-T-CANT-AX-122-r1e-08 --show=RF-SKEW-T-CANT-AX-122-r1e-12 --show=RF-SKEW-T-CANT-OFF-345-r1e-04 --show=RF-SKEW-T-CANT-OFF-345-r1e-08 --show=RF-SKEW-T-CANT-OFF-345-r1e-12 --show=RF-SKEW-T-CANT-OFF-122-r1e-04 --show=RF-SKEW-T-CANT-OFF-122-r1e-08 --show=RF-SKEW-T-CANT-OFF-122-r1e-12 --show=RF-SKEW-A-CANT-AX-345-r1e-04 --show=RF-SKEW-A-CANT-AX-345-r1e-08 --show=RF-SKEW-A-CANT-AX-345-r1e-12 --show=RF-SKEW-A-CANT-AX-122-r1e-04 --show=RF-SKEW-A-CANT-AX-122-r1e-08 --show=RF-SKEW-A-CANT-AX-122-r1e-12 --show=RF-SKEW-A-CANT-OFF-345-r1e-04 --show=RF-SKEW-A-CANT-OFF-345-r1e-08 --show=RF-SKEW-A-CANT-OFF-345-r1e-12 --show=RF-SKEW-A-CANT-OFF-122-r1e-04 --show=RF-SKEW-A-CANT-OFF-122-r1e-08 --show=RF-SKEW-A-CANT-OFF-122-r1e-12 > '<RAW_LOGS>/VK-F17-records-RF-SKEW/fault.stdout' 2> '<RAW_LOGS>/VK-F17-records-RF-SKEW/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-SKEW --show=RF-SKEW-T-PIN-AX-345-r1e-04 --show=RF-SKEW-T-PIN-AX-345-r1e-08 --show=RF-SKEW-T-PIN-AX-345-r1e-12 --show=RF-SKEW-T-PIN-AX-122-r1e-04 --show=RF-SKEW-T-PIN-AX-122-r1e-08 --show=RF-SKEW-T-PIN-AX-122-r1e-12 --show=RF-SKEW-T-PIN-OFF-345-r1e-04 --show=RF-SKEW-T-PIN-OFF-345-r1e-08 --show=RF-SKEW-T-PIN-OFF-345-r1e-12 --show=RF-SKEW-T-PIN-OFF-122-r1e-04 --show=RF-SKEW-T-PIN-OFF-122-r1e-08 --show=RF-SKEW-T-PIN-OFF-122-r1e-12 --show=RF-SKEW-T-CANT-AX-345-r1e-04 --show=RF-SKEW-T-CANT-AX-345-r1e-08 --show=RF-SKEW-T-CANT-AX-345-r1e-12 --show=RF-SKEW-T-CANT-AX-122-r1e-04 --show=RF-SKEW-T-CANT-AX-122-r1e-08 --show=RF-SKEW-T-CANT-AX-122-r1e-12 --show=RF-SKEW-T-CANT-OFF-345-r1e-04 --show=RF-SKEW-T-CANT-OFF-345-r1e-08 --show=RF-SKEW-T-CANT-OFF-345-r1e-12 --show=RF-SKEW-T-CANT-OFF-122-r1e-04 --show=RF-SKEW-T-CANT-OFF-122-r1e-08 --show=RF-SKEW-T-CANT-OFF-122-r1e-12 --show=RF-SKEW-A-CANT-AX-345-r1e-04 --show=RF-SKEW-A-CANT-AX-345-r1e-08 --show=RF-SKEW-A-CANT-AX-345-r1e-12 --show=RF-SKEW-A-CANT-AX-122-r1e-04 --show=RF-SKEW-A-CANT-AX-122-r1e-08 --show=RF-SKEW-A-CANT-AX-122-r1e-12 --show=RF-SKEW-A-CANT-OFF-345-r1e-04 --show=RF-SKEW-A-CANT-OFF-345-r1e-08 --show=RF-SKEW-A-CANT-OFF-345-r1e-12 --show=RF-SKEW-A-CANT-OFF-122-r1e-04 --show=RF-SKEW-A-CANT-OFF-122-r1e-08 --show=RF-SKEW-A-CANT-OFF-122-r1e-12 > '<RAW_LOGS>/VK-F17-records-RF-SKEW/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F17-records-RF-SKEW/NONE_after.stderr'
```

## VK-F17-records-RF-CANCEL

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_f17_cancel_diagnostic_01/D02_F17/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-CANCEL --show=RF-CANCEL-F-G1e5-GnG --show=RF-CANCEL-F-G1e5-GGn --show=RF-CANCEL-F-G1e5-nGG --show=RF-CANCEL-F-G1e6-GnG --show=RF-CANCEL-F-G1e6-GGn --show=RF-CANCEL-F-G1e6-nGG --show=RF-CANCEL-F-G1e7-GnG --show=RF-CANCEL-F-G1e7-GGn --show=RF-CANCEL-F-G1e7-nGG --show=RF-CANCEL-F-G1e8-GnG --show=RF-CANCEL-F-G1e8-GGn --show=RF-CANCEL-F-G1e8-nGG --show=RF-CANCEL-F-G1e8-GnG-ORTHO --show=RF-CANCEL-F-G1e8-GnG-INPLANE --show=RF-CANCEL-F-G1e80-GnG --show=RF-CANCEL-F-G1e80-GGn --show=RF-CANCEL-F-G1e80-nGG --show=RF-CANCEL-F-G1e80-GnG-ORTHO --show=RF-CANCEL-F-G1e80-GnG-INPLANE --show=RF-CANCEL-M-G1e5-GnG --show=RF-CANCEL-M-G1e5-GGn --show=RF-CANCEL-M-G1e5-nGG --show=RF-CANCEL-M-G1e6-GnG --show=RF-CANCEL-M-G1e6-GGn --show=RF-CANCEL-M-G1e6-nGG --show=RF-CANCEL-M-G1e7-GnG --show=RF-CANCEL-M-G1e7-GGn --show=RF-CANCEL-M-G1e7-nGG --show=RF-CANCEL-M-G1e8-GnG --show=RF-CANCEL-M-G1e8-GGn --show=RF-CANCEL-M-G1e8-nGG --show=RF-CANCEL-M-G1e8-GnG-ORTHO --show=RF-CANCEL-M-G1e8-GnG-INPLANE --show=RF-CANCEL-M-G1e80-GnG --show=RF-CANCEL-M-G1e80-GGn --show=RF-CANCEL-M-G1e80-nGG --show=RF-CANCEL-M-G1e80-GnG-ORTHO --show=RF-CANCEL-M-G1e80-GnG-INPLANE > '<RAW_LOGS>/VK-F17-records-RF-CANCEL/NONE_before.stdout' 2> '<RAW_LOGS>/VK-F17-records-RF-CANCEL/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-F17 '<ARTIFACT:vk_records-release-seeded>' RF-CANCEL --show=RF-CANCEL-F-G1e5-GnG --show=RF-CANCEL-F-G1e5-GGn --show=RF-CANCEL-F-G1e5-nGG --show=RF-CANCEL-F-G1e6-GnG --show=RF-CANCEL-F-G1e6-GGn --show=RF-CANCEL-F-G1e6-nGG --show=RF-CANCEL-F-G1e7-GnG --show=RF-CANCEL-F-G1e7-GGn --show=RF-CANCEL-F-G1e7-nGG --show=RF-CANCEL-F-G1e8-GnG --show=RF-CANCEL-F-G1e8-GGn --show=RF-CANCEL-F-G1e8-nGG --show=RF-CANCEL-F-G1e8-GnG-ORTHO --show=RF-CANCEL-F-G1e8-GnG-INPLANE --show=RF-CANCEL-F-G1e80-GnG --show=RF-CANCEL-F-G1e80-GGn --show=RF-CANCEL-F-G1e80-nGG --show=RF-CANCEL-F-G1e80-GnG-ORTHO --show=RF-CANCEL-F-G1e80-GnG-INPLANE --show=RF-CANCEL-M-G1e5-GnG --show=RF-CANCEL-M-G1e5-GGn --show=RF-CANCEL-M-G1e5-nGG --show=RF-CANCEL-M-G1e6-GnG --show=RF-CANCEL-M-G1e6-GGn --show=RF-CANCEL-M-G1e6-nGG --show=RF-CANCEL-M-G1e7-GnG --show=RF-CANCEL-M-G1e7-GGn --show=RF-CANCEL-M-G1e7-nGG --show=RF-CANCEL-M-G1e8-GnG --show=RF-CANCEL-M-G1e8-GGn --show=RF-CANCEL-M-G1e8-nGG --show=RF-CANCEL-M-G1e8-GnG-ORTHO --show=RF-CANCEL-M-G1e8-GnG-INPLANE --show=RF-CANCEL-M-G1e80-GnG --show=RF-CANCEL-M-G1e80-GGn --show=RF-CANCEL-M-G1e80-nGG --show=RF-CANCEL-M-G1e80-GnG-ORTHO --show=RF-CANCEL-M-G1e80-GnG-INPLANE > '<RAW_LOGS>/VK-F17-records-RF-CANCEL/fault.stdout' 2> '<RAW_LOGS>/VK-F17-records-RF-CANCEL/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-CANCEL --show=RF-CANCEL-F-G1e5-GnG --show=RF-CANCEL-F-G1e5-GGn --show=RF-CANCEL-F-G1e5-nGG --show=RF-CANCEL-F-G1e6-GnG --show=RF-CANCEL-F-G1e6-GGn --show=RF-CANCEL-F-G1e6-nGG --show=RF-CANCEL-F-G1e7-GnG --show=RF-CANCEL-F-G1e7-GGn --show=RF-CANCEL-F-G1e7-nGG --show=RF-CANCEL-F-G1e8-GnG --show=RF-CANCEL-F-G1e8-GGn --show=RF-CANCEL-F-G1e8-nGG --show=RF-CANCEL-F-G1e8-GnG-ORTHO --show=RF-CANCEL-F-G1e8-GnG-INPLANE --show=RF-CANCEL-F-G1e80-GnG --show=RF-CANCEL-F-G1e80-GGn --show=RF-CANCEL-F-G1e80-nGG --show=RF-CANCEL-F-G1e80-GnG-ORTHO --show=RF-CANCEL-F-G1e80-GnG-INPLANE --show=RF-CANCEL-M-G1e5-GnG --show=RF-CANCEL-M-G1e5-GGn --show=RF-CANCEL-M-G1e5-nGG --show=RF-CANCEL-M-G1e6-GnG --show=RF-CANCEL-M-G1e6-GGn --show=RF-CANCEL-M-G1e6-nGG --show=RF-CANCEL-M-G1e7-GnG --show=RF-CANCEL-M-G1e7-GGn --show=RF-CANCEL-M-G1e7-nGG --show=RF-CANCEL-M-G1e8-GnG --show=RF-CANCEL-M-G1e8-GGn --show=RF-CANCEL-M-G1e8-nGG --show=RF-CANCEL-M-G1e8-GnG-ORTHO --show=RF-CANCEL-M-G1e8-GnG-INPLANE --show=RF-CANCEL-M-G1e80-GnG --show=RF-CANCEL-M-G1e80-GGn --show=RF-CANCEL-M-G1e80-nGG --show=RF-CANCEL-M-G1e80-GnG-ORTHO --show=RF-CANCEL-M-G1e80-GnG-INPLANE > '<RAW_LOGS>/VK-F17-records-RF-CANCEL/NONE_after.stdout' 2> '<RAW_LOGS>/VK-F17-records-RF-CANCEL/NONE_after.stderr'
```

## VK-S1-rf_skew

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_s1_retarget_01/D02_S1/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_skew --test-threads=2 --nocapture > '<RAW_LOGS>/VK-S1-rf_skew/NONE_before.stdout' 2> '<RAW_LOGS>/VK-S1-rf_skew/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-S1 '<ARTIFACT:lane-debug-seeded>' --exact rf_skew --test-threads=2 --nocapture > '<RAW_LOGS>/VK-S1-rf_skew/fault.stdout' 2> '<RAW_LOGS>/VK-S1-rf_skew/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_skew --test-threads=2 --nocapture > '<RAW_LOGS>/VK-S1-rf_skew/NONE_after.stdout' 2> '<RAW_LOGS>/VK-S1-rf_skew/NONE_after.stderr'
```

## VK-S2-rf_finite

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_s2_retarget_01/D02_S2/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_finite --test-threads=2 --nocapture > '<RAW_LOGS>/VK-S2-rf_finite/NONE_before.stdout' 2> '<RAW_LOGS>/VK-S2-rf_finite/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-S2 '<ARTIFACT:lane-debug-seeded>' --exact rf_finite --test-threads=2 --nocapture > '<RAW_LOGS>/VK-S2-rf_finite/fault.stdout' 2> '<RAW_LOGS>/VK-S2-rf_finite/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:lane-debug-seeded>' --exact rf_finite --test-threads=2 --nocapture > '<RAW_LOGS>/VK-S2-rf_finite/NONE_after.stdout' 2> '<RAW_LOGS>/VK-S2-rf_finite/NONE_after.stderr'
```

## VK-R02-records-RF-LARGE

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_10/P49/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-LARGE --show=RF-LARGE-TREE-n00100-AX > '<RAW_LOGS>/VK-R02-records-RF-LARGE/NONE_before.stdout' 2> '<RAW_LOGS>/VK-R02-records-RF-LARGE/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-R02 '<ARTIFACT:vk_records-release-seeded>' RF-LARGE --show=RF-LARGE-TREE-n00100-AX > '<RAW_LOGS>/VK-R02-records-RF-LARGE/fault.stdout' 2> '<RAW_LOGS>/VK-R02-records-RF-LARGE/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-LARGE --show=RF-LARGE-TREE-n00100-AX > '<RAW_LOGS>/VK-R02-records-RF-LARGE/NONE_after.stdout' 2> '<RAW_LOGS>/VK-R02-records-RF-LARGE/NONE_after.stderr'
```

## VK-R28-records-RF-LARGE

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_10/P52/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-LARGE --show=RF-LARGE-CHAIN-n00100-AX --show=RF-LARGE-TREE-n00100-AX > '<RAW_LOGS>/VK-R28-records-RF-LARGE/NONE_before.stdout' 2> '<RAW_LOGS>/VK-R28-records-RF-LARGE/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-R28 '<ARTIFACT:vk_records-release-seeded>' RF-LARGE --show=RF-LARGE-CHAIN-n00100-AX --show=RF-LARGE-TREE-n00100-AX > '<RAW_LOGS>/VK-R28-records-RF-LARGE/fault.stdout' 2> '<RAW_LOGS>/VK-R28-records-RF-LARGE/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:vk_records-release-seeded>' RF-LARGE --show=RF-LARGE-CHAIN-n00100-AX --show=RF-LARGE-TREE-n00100-AX > '<RAW_LOGS>/VK-R28-records-RF-LARGE/NONE_after.stdout' 2> '<RAW_LOGS>/VK-R28-records-RF-LARGE/NONE_after.stderr'
```

## VK-UNKNOWN-adapter

Historical source: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I22/vk_runtime_09/P37/EVIDENCE.json. Fresh argv0 must be rebound; all later arguments are exact.

```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:adapter-debug-seeded>' --exact permuting_every_list_changes_no_canonical_byte --test-threads=2 --nocapture > '<RAW_LOGS>/VK-UNKNOWN-adapter/NONE_before.stdout' 2> '<RAW_LOGS>/VK-UNKNOWN-adapter/NONE_before.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=VK-UNKNOWN '<ARTIFACT:adapter-debug-seeded>' --exact permuting_every_list_changes_no_canonical_byte --test-threads=2 --nocapture > '<RAW_LOGS>/VK-UNKNOWN-adapter/fault.stdout' 2> '<RAW_LOGS>/VK-UNKNOWN-adapter/fault.stderr'
```
```text
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 FK_SEEDED_FAULT=NONE '<ARTIFACT:adapter-debug-seeded>' --exact permuting_every_list_changes_no_canonical_byte --test-threads=2 --nocapture > '<RAW_LOGS>/VK-UNKNOWN-adapter/NONE_after.stdout' 2> '<RAW_LOGS>/VK-UNKNOWN-adapter/NONE_after.stderr'
```

