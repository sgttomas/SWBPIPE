# Prepared commands — NOT executed

All commands require a new ROOT execution grant after disposition of the separately reported I21 Uc-c survivor. Guard and slot checks come from COMMON. Use one cargo process, installed 1.97.1/auto-install0/offline/locked/incremental0, -j4, tests2, and a designated separate target. Preparation used the exact VENV in BASIS.json; no compiler, test binary or model was launched.

For NONE and each individual VR-M01..M06 postimage, materialize a separate owned archive of 10315a8167c47f43aa41402beb88ed2c70e62cf2. Apply only its saved diff to VR/src/envelope.rs; verify the exact input/postimage hashes in MUTANTS.json. NONE is the unmodified source (empty NONE.diff). Never mutate the maintained checkout or apply variants cumulatively.

From that archive's VR directory, with CARGO_TARGET_DIR bound to the authorized slice:

    env GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 cargo test --offline --locked -j4 --test k6c_envelope --no-run
    env GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 cargo test --offline --locked -j4 --test k6c_envelope all_193_independently_reviewed_reference_values_and_source_drop -- --exact --test-threads=2 --nocapture

First build must succeed. Capture its exact artifact/fingerprint/source/archive hashes and all stdout/stderr. Compile error, unrelated panic, timeout, or malformed application gives no kill. A required survivor stops its affected path. NONE must pass before and after the corresponding mutated source. M02/M05 presently have pending nested-arm witness cells; do not claim them killed or substitute a larger deletion.

The original 15 seeded registrations use the unchanged run_seeded_faults.py FAULTS registry and existing seeded sites. A future same-candidate full mechanical matrix command is:

    env GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 <VENV> runner/run_seeded_faults.py --out <GRANTED_OUTPUT> --timeout 1800

Its NONE and VK-UNKNOWN controls remain required. Its any-failing-test verdict is not sufficient semantic credit after the new envelope tests exist. SELECTION.json gives all 15 actual qualified routes, exact original command argv/filter/profile environments and same-filter NONE returns. Rebuild/rebind the same named test/example target on the deduplicated archive, replace only argv[0] with its newly emitted matching artifact, preserve all other args, fault selection, control order and complete original test assertions. Current artifact/profile/hash binding is pending. Do not reuse an old binary by pathname.

For example the existing ordinary value witness is the freshly bound lane test artifact with:

    FK_SEEDED_FAULT=VK-F02 <SEEDED_LANE_TEST_BINARY> --exact rf_chain --test-threads=2 --nocapture

Its NONE runs use the same argv/filter/artifact and FK_SEEDED_FAULT=NONE. The existing R02 record witness uses:

    FK_SEEDED_FAULT=VK-R02 <SEEDED_RELEASE_VK_RECORDS> RF-LARGE --show=RF-LARGE-TREE-n00100-AX

The show flag filters printing only; preserve full 12-case computation/parity and same-filter NONE controls. Use the exact P/D commands under SELECTION.json for the other cases. Do not turn generic work/record changes, new envelope failures, an unexecuted later assertion, or an unrelated Ceiling into the registered witness. F17's owner-adopted relative-certificate route is F17-only and retains source-linked named predicates, complete outcomes and restoring controls; normal numerical/class checks remain mandatory.

No new matrix framework, driver or test formula is supplied. Existing source/harness scripts were only read. Historical real durations, where directly present in each original stderr, are recorded in ORIGINAL_WITNESS_COMMANDS.json; null means unavailable. They are source-backed prior observations, not predictions for a future dedup build. No unsupported duration estimate is made.
