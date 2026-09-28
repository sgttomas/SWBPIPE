# I8R: resume K1 on the owner's Mac (fresh TASK after the handoff)

This is an implementation TASK that continues I8's K1 work. I8 stopped at the handoff and committed its drafts as one WIP commit. Nothing was compiled or run.

Read, in this order:
1. Root `AGENTS.md` and `agents/AGENT_TASK.md`.
2. `_COMMON.md`. **This brief overrides its "Host resources" section** (see "The Mac host" below).
3. `I8_K1_IMPLEMENTATION.md`, with addenda 1–4. It remains your brief.
4. `HANDOFF_2026-09-28_TO_LOCAL.md` §§4–5.
5. On the K1 branch: `T3/IMPLEMENTATION/K1/WIP_STATE.md`. Read every draft it lists in full before building; they are yours to finish, not to trust.

Make no Git writes and no index operations (no commit, checkout, restore, reset, stash or push). ROOT commits.

## Roles on the Mac

- **ROOT (HELP_HUMAN) dispatches you directly and is your return path.** For now there is no separate T3 manager on the Mac. ROOT verifies your work, commits it, and rules on anything that touches a ROOT ruling or pin.
- **Delegation mechanism:** a background subagent of ROOT's session, started with this brief. Record this in RETURN.

## Where things stand

- **Branch and head:** `codex/piping-k1-20260928` in `<wt>/k1`, at the WIP commit `d08b0efc7` ("K1 WIP (handoff; not reviewed)"), based on main `134eefc24`.
- **K2a** is finishing in the cloud session. Its PR is not yet merged. K1's PR cannot merge before K2a's.

## The Mac host (overrides `_COMMON.md` "Host resources")

**This Mac has no swap.** On 2026-09-28 it crashed, a kernel watchdog panic from memory exhaustion, when ROOT ran 12 gate probes in parallel without a memory cap. Memory is the binding constraint:
- **Cargo:** `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`.
  - Use `-j 8` at most, and `RUST_TEST_THREADS=4`.
  - Run at most two cargo jobs of your own at once.
  - Use your own target `<wt>/k1-target`.
- **Never materialize a dense matrix for a model with 10,000 or more members.** A dense n×n array at 60,000 DOFs is about 29 GB.
  - The RF-LARGE storage counts at 10,000 members must go through the pattern path only.
  - Dense and sparse parity tests stop at 1,000 members.
  - If a test could reach a dense path at that size, guard it and say so in RETURN.
- **The memory guard.** A host guard SIGKILLs T3 processes (anything whose command line names `<wt>`) if available memory falls below 35%. It logs to `<wt>/guard/memguard.log`. If something of yours dies with SIGKILL, check that log and report it; do not retry blindly.
- **Mutations:** one clean copy and one clean target per mutant, under `<wt>/k1-mut/<mutant>/`, and delete each target afterwards. Run at most three mutants at once, each at `-j 4`.
- **Disk** is not tight (over 1 TB free), but prune your targets when you are done.
- **No timing or memory-growth claims** (K6 owns them).
- **rustfmt:** use the stable toolchain's rustfmt on your new and changed files only, as I8 did.
- **Python:** `<VENV>` is the repository venv with `requirements-dev.txt` (ROOT gives the path at spawn).

## Platform calibration (ROOT, 2026-09-28): what it means for K1

- **T9 is not platform-independent.** On main, 100 of 112 committed-fixture outputs are byte-identical between this Mac and the Linux records.
  - The other 12 differ only through the platform libm. macOS `hypot` and `expm1` are 1 ulp from correctly rounded on 16 of 334 and 1 of 10 distinct arguments; `exp` agrees.
  - Replaying correctly rounded results for those calls reproduces all 112 Linux hashes.
- **So build T9's base and candidate on this Mac, from `git archive` copies, and compare them with each other.** Never compare a Mac output with a Linux hash record.
  - ROOT's Mac main hashes are in `<wt>/scratch/calib/fixdiff/sha_native.txt`, from main `649162522`, whose piping tree equals `134eefc24`'s. They are a cross-check for your base build, not a substitute for it.
- **Suites.** ROOT runs CI's 39-manifest cargo profile on main on this Mac and gives you the result (`<wt>/scratch/calib/suites_main/`).
  - A test that fails on Mac main fails for a platform reason, for example a committed Linux byte. Report it; do not "fix" it in product code.
  - K1 must add no failure: every test failing on K1 must fail identically on Mac main.

## What to do

Follow `WIP_STATE.md` §4:
- steps 1–5 now: compile check; targeted tests; suites; T9; mutations;
- step 7: the records;
- step 6 (the K2a-interaction tests and the combined-tree re-run) only after ROOT merges main, with K2a, into the K1 branch and tells you.

**Checkpoints.** End your turn with a status message to ROOT at each one; ROOT verifies, commits and resumes you:
- **A.** The compile is clean and green: the targeted `k1_`/`krev0` tests, the existing pins, `s11_site_table` and PP `s11f_site_test`. Or a stop.
- **B.** The suites (per crate, against the Mac main baseline) and T9 (Mac base against Mac candidate).
- **C.** The mutation table, with the NONE control first, and the original pins' mutants with their kill sites.
- **D.** CHANGE_RECORD and RETURN drafted, with `_run_records/` and SHA256SUMS, and no machine paths.

At each checkpoint, list the changed files, and say which hunks belong to which commit:
- the K1 slice;
- the separate `formation_check.rs` site-table commit;
- the KERNEL-list hunk.

**Stop and report** (end your turn) in any of the cases the I8 brief and addenda name. These include:
- a committed-byte change in T9;
- a bitwise parity failure;
- a needed edit to `FK/lib.rs` or `PP`;
- a site that fits no disposition;
- a plain binary64 load, force or RHS fold in `formation_check.rs`;
- an original pin mutant that is no longer killed;
- a design item that cannot be implemented as specified.

## Records

- Put records under `T3/IMPLEMENTATION/K1/` on the K1 branch, with placeholders only (`<wt>`, `<scratch>`, `<VENV>`, `<home>`).
- Use no machine paths and no model identifiers.
- Record the platform: `aarch64-apple-darwin`, rustc 1.97.1.
- Records that name T9 must say it is a Mac-only comparison.
