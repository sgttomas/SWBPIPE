# I22 G0 build return

**PASS — one authorized build completed, exit 0. No solver case ran.**
No owned build is running. Return to recovery manager/ROOT for verification and
a separate B grant; no numerical or publication claim is made by this build.

## Authority and identities

Same resumed TASK `/root/t3_recovery_manager/i22_a1`, parent
`/root/t3_recovery_manager`, native `collaboration` mechanism; no delegation.
G0 grant: COORD commit `90b6bcbbf64b13975211038bf3f33bb87273e646`,
R/BRIEFS/I22_G0_BUILD.md SHA256
`2c1c6e0afdd5bd1a0a96623ca75e2c7084dc8503ab0f5318e23a244e9cc8706b`.
The previously loaded TASK/project/COMMON/skill basis remains recorded in
../BASIS.json. G0 extends only the named build fence, not repair/run authority.

Aliases: WT is ROOT's supplied t3 host directory; A1_WT=WT/a1;
R is the existing RESUME_2026-09-30 path; E=A1_WT/R/I22/build_01;
S=WT/scratch/i22; HOME/CARGO_HOME/RUSTUP_HOME are the owner's existing bindings.
No machine absolute path is embedded in this durable packet.

Product HEAD remains `d01ad98a754698631f927709d08284c272de85e8`.
Archived FK source is `3bddc2b05f6106e969c7cf43373b230845c7cc66`; complete FK
subtree diff against product HEAD is empty. Source archive/copies were made with
GIT_OPTIONAL_LOCKS=0 on every Git invocation. Probe and lock came from immutable
response `520d7dfb790bcedabc03e92b9692884ce295be54`. The probe's truthful
SOURCE_COMMIT field is unchanged. SOURCE_SHA256SUMS hashes all 115 archived
FK files, and all verify after the build.

## Executed job and supervision

Tool-call launch timestamp: **2026-10-01T04:58:43.989Z**.
Tool-managed PTY session: **24126**. Initial 1-second yield returned the running
session; the next supported write_stdin poll returned **exit 0**. No interrupt,
retry, fallback wrapper or second build occurred. The command ran from S/probe:

```sh
/usr/bin/time -l env -u FK_SEEDED_FAULT -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=<WT>/scratch/i22/target "$HOME/.cargo/bin/cargo" build --offline --locked --manifest-path <WT>/scratch/i22/probe/Cargo.toml --bin a1_public_probe --no-default-features -j 4 --verbose > <WT>/scratch/i22/build.stdout 2> <WT>/scratch/i22/build.stderr
```

This is the sole Cargo build. Output redirection retains complete stdout and
stderr in S while the job remains owned by the supported PTY. Stdout is empty;
stderr includes the complete verbose compiler commands and /usr/bin/time output.
BUILD.stdout is the unchanged empty copy; BUILD.stderr.portable replaces only
WT and home-root strings. Original raw files remain untouched in S.

Observed /usr/bin/time values:
- 3.80 seconds real, 3.10 user, 0.42 system.
- 430,161,920 bytes maximum resident set size.
- 7,357,040 bytes reported peak memory footprint.
- 0 swaps; all remaining raw counters are retained.
Cargo separately reports completion in 3.77 seconds.

These are the command's reported resource fields, not a proved process-group
maximum or performance bound. The existing host-wide memory floor is the only
adopted memory enforcement; no hard per-process RSS cap or automatic 300-second
deadline is claimed. The operator threshold was not reached. The host guard PID
5387 was observed immediately before and after the build, running the existing
WT/guard/memguard.sh, SHA256
`ee11ea1f43f0892dd0396bfe5c68e133084c90c87abbf757ce0be723e0ca7b79`.
This packet records before/after liveness, not a new continuous guard audit.
The post-build process query found no cargo/rustc/time/clang/cc/ld executable
whose arguments referenced S. The PTY job completed normally; no owned build
remains running.

## Configuration, features and byte identities

Preserved CARGO_HOME=<HOME>/.cargo, RUSTUP_HOME=<HOME>/.rustup and owner HOME;
no CODEX_HOME or user configuration change. Inherited RUSTUP_TOOLCHAIN=stable
was overridden only for this job by the explicit 1.97.1 environment.
RUST_LOG remained inherited. The five injection/wrapper variables in the argv
were absent at preflight and explicitly removed for the job.
Other inherited environment was preserved; no credentials were enumerated.

Both config and config.toml were checked at S/probe and each ancestor's .cargo,
plus CARGO_HOME. None existed, so there is no relevant config-file hash to
report. No --config override was supplied. Only the scratch Cargo.toml path
dependency changed from the historical snapshot location to
`../source/projects/chirality-piping/core/solver/frame_kernel`.
The historical two-package Cargo.lock was copied unchanged; no lock generation,
registry resolution, network or install was performed.

Installed rustc reports 1.97.1, commit
`8bab26f4f68e0e26f0bb7960be334d5b520ea452`, host aarch64-apple-darwin,
LLVM 22.1.6. Cargo's verbose compiler argv points to that toolchain.
Both exact copied fingerprint files report enabled features **[]** and
rustflags **[]**. Kernel declared_features contains mutation-controls, but
it is not enabled. No cfg(test) or source instrumentation was used.

| Object | SHA256 |
|---|---|
| S/probe/src/main.rs | 9e054fa80fff5c2192b1bfef5ad031d24c4d9dd3ab2407b6f9df517cc74d740a |
| S/probe/src/cases.rs | b925980a3b5b3c206e9e419f6e9fd1fffdedab7feffc1173650f92ecf2ea3b8c |
| S/probe/Cargo.toml (rebound path only) | eda372d79cdd4e368af2daa7156c63bd5e7c020f4522f8926fb0ff1446597345 |
| S/probe/Cargo.lock | ca870fd55afca7aebeadd4410bb77a2a40391ac80a7ff95c36ab6b55fe66bc4d |
| S/target/debug/a1_public_probe | bcbe897204ec702b99529d25e6d0213d0132af5e6086e0397fae3a8f8ef8a08f |
| S/build.stdout (original) | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 |
| S/build.stderr (original) | fc951bf1163cc3fce3d22f17c83d3b5d757c04cd567e991aae5ad1f1dd52d6a9 |
| FEATURES_PROBE.json | 55384b8d01b589a406fd4a729a73ed33e542d113b735b74809b7b297fdd95f79 |
| FEATURES_KERNEL.json | 93c85cff3a5cb20c392c559f29884ea9608c8e5de4265d5a2a1e7163cfcc88a5 |
| Configured .cargo/bin/cargo rustup proxy | ec1b9233e7f72990ecd8e62063fa7f6c3dfc2bec8e97f88bff165f9100ac696a |
| Actual 1.97.1 bin/cargo | 7672ead309d505577c018fff2cafb3433601f073e38cbe87359ac1f7b944bbf5 |
| Actual 1.97.1 bin/rustc | 210df6794001b73ec3d453878707fa1e0bdcb63c427024a6e6574bbe5615a4da |

## Checks, writes and limitations

COMMANDS.json retains actual sanitized argv, environment inspection, preparation,
tool outputs, PTY launch/completion and postchecks. One initial seal query used
the wrong relative path from the manifest directory and returned missing-file
exit 2; the corrected command verified all six checkpoint entries before build,
and all six verified again after build. This was a read-path error, not a
changed seal. The initially absent scratch folders were confirmed before creation.

Only E/** and S/** were written by I22. SCRATCH_SHA256SUMS is the final scratch
file inventory (source, copied probe/lock, compiler artifacts and raw logs);
SOURCE_SHA256SUMS separately identifies the unchanged archive. This packet's
SHA256SUMS inventories every durable build artifact except itself.
Tracked and cached repository diffs remain empty. No Git/index mutation,
maintained solver/harness/validation edit, I21 write, guard/config/tool change,
new wrapper or supporting-tool development occurred. The sealed checkpoint
bytes remain unchanged.

G0 proves the pinned unchanged probe compiles under this environment. It does
not establish any source model's runtime admission, complete selected solve,
oracle comparison, performance limit or publication guarantee. All solver cases
remain unrun in this resumed I22 assignment. ROOT must verify this return and
grant B separately with the frozen independent comparator before any case runs.

