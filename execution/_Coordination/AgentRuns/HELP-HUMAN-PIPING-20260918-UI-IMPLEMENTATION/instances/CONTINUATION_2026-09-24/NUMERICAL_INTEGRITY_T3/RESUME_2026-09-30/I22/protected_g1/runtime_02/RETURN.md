# G1 runtime_02 stopped before Cargo — log directory permission

G02's exact frozen K4-M34 patch applied successfully with the existing apply_patch tool. Its postimage matches:
00dc1b611ed758f092038af95dce5d99bad2f4827b05314af39c52d9853f7c9a.

The first attempted test launch then failed before /usr/bin/time, env or Cargo executed. Creating the per-copy log directory was denied because the prepared copy root is read-only (observed mode dr-xr-xr-x). The following stdout redirection could not open its missing parent. No mutant test or baseline-return control ran; no semantic kill is credited.

Raw tool output, preserved exactly except portable root aliases:

    mkdir: <G1_S>/G02/runtime_02_logs: Permission denied
     5387     1 03-11:35:45 /bin/bash <WT>/guard/memguard.sh

    zsh:1: no such file or directory: <G1_S>/G02/runtime_02_logs/logs/mutant/G02.stdout

The mkdir/guard cell was:

    mkdir -p '<G1_S>/G02/runtime_02_logs/logs/mutant'
    ps -p 5387 -o pid=,ppid=,etime=,command=

The following launch attempt at13:12:04.085Z exited1:

    /usr/bin/time -l env -u FK_SEEDED_FAULT -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR='<G1_TARGET>/G02' "$HOME/.cargo/bin/cargo" test --offline --locked -j 4 --lib 'structural::retained::bound::tests::v4s_f2_family_bounds_equal_the_emulation_and_are_never_below_the_exact_norm' -- --exact > '<G1_S>/G02/runtime_02_logs/logs/mutant/G02.stdout' 2> '<G1_S>/G02/runtime_02_logs/logs/mutant/G02.stderr'

Working directory: <G1_S>/G02/projects/chirality-piping/core/solver/frame_kernel.
No log file was created at either intended path. The actual raw evidence is the tool output from cells786108 (mkdir/guard) and9647f5 (redirection), preserved in COMMANDS.json. ps succeeding caused the shared mkdir/guard cell to exit0 despite the visible mkdir error. That orchestration oversight is disclosed; it did not start Cargo. A preceding JavaScript quote typo was fixed before this first launch and produced no tool-side action.

The G02 target remains empty; patched source is retained. No permissions, TMPDIR, environment, tool, log location or source criterion was changed after the stop. No retry, alternate path or later group was attempted. Manager confirmed the stop and requested this sealed return.

## Preserved basis and obligations

Actual release acknowledgement13:10:19 UTC; fixed deadline14:10:19. Grant87e4a9a0fe5437f53d3ebe214d934b2042d4610d remains unchanged. Manager re-confirmed containment13:11:26 before patch application. RECHECK.json binds53 original copies plus the read-only baseline,6264 original-file hashes, all28 prior baseline pass/source/test/lock/binary/fingerprint identities, no symlink/hardlink/maintained alias, and empty targets.

ROOT's separate G01 completion is bound to R/verification/tool_resume_01 sealbd19520e49b09b2f8e284e0594f61de54cce670e5d1c75563fdd4378186c8973: intended Phi assertion plus baseline/control pass. It is ROOT's result, not a repeated I22 kill. Old system-patch failure,28 G1 baselines and all original readiness/ROOT seals remain unchanged. Source basis40129a225d73860ac2a53da9a2fa73869df668f3 and frozen manifestbd4dae325f30af11bb9ec1479f255f73912b96b7d4e50a066825aee725644e90 remain authoritative.

All53 runtime_02 mutant filters and53 return controls are UNRUN. G03–G54 remain unapplied. No maintained source/test/lock/assertion/input/truth, observations, Git/index, new diagnostic/framework or child. No owned job is running. ROOT must disposition the log destination and explicitly regrant continuation; no correction is performed here.

