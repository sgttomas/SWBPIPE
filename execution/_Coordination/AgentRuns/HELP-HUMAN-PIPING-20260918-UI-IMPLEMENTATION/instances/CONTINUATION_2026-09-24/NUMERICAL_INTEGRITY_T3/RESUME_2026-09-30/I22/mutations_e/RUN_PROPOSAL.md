# Proposed finite runtime batch — NO runtime authority in E

Candidate40129a225d73860ac2a53da9a2fa73869df668f3, numerical helpers dd1-identical. ROOT first reviews the 35 exact patches, MANIFEST.json, and this proposal, then supplies a runtime slot/target/timebox. All source/tests/locks/oracles remain frozen. No patch or test was executed in E; PATCH_CHECKS.json is text-application checking only.

## E1: new publication mutations

Fixed order is MANIFEST.json's 35-entry order, PM01 through PM18b. There are14 unique unmutated filters,41 mutant-filter invocations including the three PC40/41/42 filters for each PM16 variant, and35 post-mutant primary-filter baseline controls:90 focused test invocations in total if nothing stops. Build occurs as needed by these exact Cargo commands; no broad suite or probe is included.

1. Materialize a read-only FK baseline from the pinned Git archive, retaining its Cargo.lock. Use an independently disposable copy per mutant; no worktree/Git/index changes. Source/test/lock hashes and the actual target bindings must be recorded before the first run.
2. Run the14 unique baseline filters exactly once with seeded-fault env unset. Each must execute exactly one test and pass. Any baseline failure stops the batch before mutations; no old record or expected assertion may be refreshed.
3. Before each variant, verify patch hash, baseline source hash, fixed test hash and guard. The frozen task-local applicator can verify/apply precisely one patch:

       <VENV>/bin/python -B <E>/apply_frozen_patch.py <E>/MANIFEST.json <ID> <DISPOSABLE> --apply

   It refuses a Git checkout root, requires the exact preimage/patch/postimage hashes and writes only the named file. It runs no Rust. Its check-only mode was exercised on all35 patches against the immutable reference archive.
4. For each listed filter, use the exact manifest argv, with cwd the disposable FK crate. Wrap it in the inherited environment and supervision:

       /usr/bin/time -l env -u FK_SEEDED_FAULT -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER GIT_OPTIONAL_LOCKS=0 RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=<OWNED_MUTANT_TARGET> <CARGO> test --offline --locked -j 4 --lib <FILTER> -- --exact

   ROOT binds the actual target/working directory before runtime. Existing guard, tool-managed PTY, time-l and one Cargo at a time remain required. No per-process hard cap, automatic deadline, new wrapper or performance claim is implied.
5. Require compilation, a single executed named test, and its predeclared semantic assertion. An unrelated failure, compile error, test-filter miss, timeout, policy/golden snapshot drift, or earlier certificate masking is NOT a kill. A primary discriminator that unexpectedly passes is a survivor/stop, not permission to search for a different failing test.
6. PM16a omits only clone-delta aggregation; PM16b merges the whole clone counter; PM16c omits only reaches scratch aggregation. PC40/41 use the unchanged correct frozen ledgers. No wrong-cost prediction is substituted. PC42 is expected to remain green for PM16c because these terminal routes never enter reaches; that negative control does not make PM16c a survivor if its required PC40/41 assertions fail. PC42's clone-sensitive assertions are intended failures for16a/b.
7. Preserve the mutated source hash, patch, binary/features, exact argv/env, full output, semantic failure and resources before leaving the variant. Return to the untouched baseline archive/binary, reverify its hashes and run that variant's primary filter once as restoration control. Do not infer restoration from a Git reset or silently delete the mutant evidence.
8. Stop immediately on false unmutated publication, invalid identity, missing supervision/guard, protected standing change, numerical/accounting issue outside the intended isolated fault, or a surviving required variant. All later entries remain explicitly UNRUN.

The positive-radius zero/swap variants use0<bits<ABSENT_RADIUS_BITS. The wrong-index accessor variant leaves storage intact and incorrectly reuses the first positive present radius; M:1's explicit accessor tuple distinguishes it from D:6. PM15 class and bound are separate patches. PM17 uses the fixed prescribed-dominant/O9 absent-value controls; PM18b uses the independent409-set classification corpus.

## Protected follow-on inventory, not an E1 expansion

PROTECTED_FAULTS.json preserves88 R7/K4 registered entries (including NONE/NONE-S11),10 final RV19 entries,15 seeded V-K IDs plus NONE/VK-UNKNOWN controls,11 original A2/KF3 faults plus its NONE control, and both seven-entry RV23 registries. Duplicate named IDs across histories retain their separate origins. Original source hashes, exact historical edits, registered tests and commands remain in that file.

- R7/K4 historical driver pins8f8023a20; RV19 final driver pins5a46a6278. Do not execute them unchanged and relabel their result as40129a. Seventeen R7/K4 entries have anchor drift on the current source and require explicit patch adaptation/review before execution. Matching text anchors alone still do not prove a patch compiles or preserves intended semantics.
- Four R7 derivation guards retain their original expected status; R7-M14's nonbehavioural/derived disposition and historically retired M19 are explicitly recorded from the accepted plan. Nothing is newly declared equivalent, retired or dropped.
- RV19-D4u's historical underflow isolation deletes two protected test assertions. E does not authorize that edit. Preserve the obligation and obtain ROOT's exact disposition; do not silently substitute D4, claim the same kill, or remove the protected checks.
- Original A2 M1–M8 (including M4a–d) are registered in KF3 CHECKPOINT_A §4. Its bounded _run_records/a directory contains results, not the original exact patch script. The semantic/test map is preserved; recovery/freezing of those original edits remains a specific dependency. The later RV23 variants do not erase these IDs.
- The existing V-K runner builds with seeded-faults, then runs broad VR tests for NONE, each of15 faults and VK-UNKNOWN. Its exact commands and semantic targets are recorded, with proposed focused per-ID commands clearly marked for ROOT review. They are outside E1 and this grant. Any baseline policy/record drift needs disposition before fault credit; no observation refresh or broad H/VR run is proposed here.
- Historical runners sometimes call any failed test a kill; the RV23 confirmation script even includes compile failure. Those verdict policies do not satisfy current acceptance. Preserve the registered semantic target and exact failure, even when the new gate masks an old fault.

No broad H/VR, generator, --write, scale, new raw case, parameter sweep or mutable oracle is part of E1. ROOT can grant subsequent named protected batches after their concrete adaptation/coverage dependencies are resolved.

