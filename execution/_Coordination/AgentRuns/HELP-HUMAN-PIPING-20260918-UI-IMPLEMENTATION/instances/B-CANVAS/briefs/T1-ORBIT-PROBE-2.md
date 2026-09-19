# Sealed brief — T1-ORBIT-PROBE-2: finish the lane's guidance probe from its pause state

Sealed by B-CANVAS (WORKING_ITEMS, Type 1, the canvas lane manager of Tranche B; its second manager) on 2026-09-19 before launch. Role of the reader: TASK (Type 2). Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type, foreground. **You work alone and never delegate: launch no agent.** Your parent is B-CANVAS; corrections reach you by message with your agent id.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the git worktree your launch message names; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{DESKTOP}` is `{WORKING_ROOT}/apps/desktop`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{LANE}` is `{RUN}/instances/B-CANVAS`. Write no absolute machine path in any file you author, in any output file the tool writes, or in your return. Work only in the worktree you are given. **Run no git command that changes state** (no add, commit, stash, checkout, reset, merge, rebase, push); your parent commits. On this host the file-writing tools refuse paths in that worktree: write files through the shell (heredocs or short exact-once scripts), each file whole in one step.

## Who came before you

A child named T1-ORBIT-PROBE built the tool and was paused by the owner's session limit. It belonged to an earlier session and cannot be resumed; you finish its task from its state. Read, and verify each hash with `shasum -a 256` before you rely on it:

- Its sealed brief, `{LANE}/briefs/T1-ORBIT-PROBE.md`, SHA-256 `9f67f88e696ada952fbb06eafab347f99c11403b1209e6b250be9b3bb2d2e6e1`. **Read it whole. It stays in force: its "Limits that bind this work" section binds you in full, its subcommand interface is a contract, its evidence list is yours to produce, its checks are yours to run, and its return is the return you give.** Where this brief is narrower or later, this brief governs.
- Its pause status, `{LANE}/returns/T1-ORBIT-PROBE_PAUSE_STATUS.md`, SHA-256 `690024b74571d8c7c8703f9059fb97ad31012e33779e06b0a2ef3dde9cc1f78c`: what works, what has been run, what is not done.
- The tool's own notice, `{LANE}/tools/STATUS.md`, SHA-256 `9c17065d0adabe29152a3beead255e7f8d11ae4d3ecb9f92eb16035aede1f545`, including its "Found so far" observations, which are trial observations and not evidence.
- The tool, `{LANE}/tools/orbit_probe.mjs`, SHA-256 `ac5290bcb93fba13f45c8f198b6cb13ee617a2b52766426cdc81f1d344598368`, 1,560 lines. Read it whole before running it: you are answerable for its method.

One addition was made to the sealed brief by message on 2026-09-19T04:08Z and is in the tool (`{LANE}/briefs/_INDEX.md`, "Additions by message"): a variant may name its own build, `<name>[@<distDir>]=<query>`, so that a comparison can be between two build copies interleaved in one series; `--dist` is optional when every variant names one; each run is served only from its own build; every variant's build identity is recorded; `summarize` prints each identity once and says whether the variants share it; the self-test covers two copies of one build and a directory without `index.html`. It is part of the interface and belongs in the README.

## What has changed since the pause

- The lane merged `origin/main`, which carries the shell lane's slice B2. The product's layout and its controls may have moved: the earlier finding that a window of 1646 × 1168 gives a canvas of exactly 1000 × 828 CSS px must be **re-measured**, and anything the tool relies on in the page (control names, test ids, the diagnostics surface `globalThis.__openPipeStressUiDiagnosticsV1.readCurrent()`, the open-fixture route) must be re-verified against the build you measure. The self-test is your first evidence of that. If the product moved under the tool, patch the tool (it is in your write scope), say exactly what and why, and re-run the self-test after every patch.
- The build you measure is a copy the manager made outside the repository at the lane head `b415b43df`; your launch message names its path. Its `index.html` has SHA-256 `e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb`; check it before the first run and record it in `SUMMARY.md`. **You run no build**, and you never serve or read `{DESKTOP}/dist`, because another child rebuilds it while you work.
- The self-test passed once before four later patches (a blank-page pacing calibration, a histogram of pointer send gaps, the camera-sequence change in the `resources` orbit control, a garbage collection in the tool's own process before each orbit) and has not been re-run since. Re-run it first.

## Working beside another child

A second child, C1E-EDGE, works in this same worktree at the same time. It edits product files under `{DESKTOP}/src/features/viewport/**`, runs unit suites, `tsc` and builds **through the shared lock** so that they cannot overlap a timed run of yours, and later runs your tool on its own builds. So:

- Every browser you launch, every server you start and every series you run goes through `sh {RUN}/tools/with_e2e_lock.sh <command>`, one series per hold, each hold under about fifteen minutes. Waiting for the lock is normal.
- Record the host's load averages beside every series (the tool does); this host is shared by other lanes and by the other child, and only browser and timed work is behind the lock.
- Your write scope is unchanged: `{LANE}/tools/**` and `{LANE}/probes/T1/**`. Nothing under `{DESKTOP}/**`, nothing else under `{LANE}`.
- **Your last two acts before the return are: write `{LANE}/tools/README.md`, then remove `{LANE}/tools/STATUS.md`.** C1E-EDGE measures only once the README exists and `STATUS.md` is gone, so do not write the README early or remove the notice early. Do not patch the tool after the README is written; if you must, restore `STATUS.md` while you do and remove it again after.
- Port 5185 only. Never bind 5174, 5175, 5176, 5177, 5183, 5184 or 5186.

## What to produce

Exactly the sealed brief's evidence, no more: the saved `self-test` output; the four A/A series at 10,000 pipes (each geometry mode, each pacing; light, labels on, device pixel ratio 2, window 1440 × 900, `--variants "a=;b=" --pairs 3`); the two `vsync` A/A series at 1,000 pipes; `resources` at 1,000 and at 10,000 pipes; `SUMMARY.md` with the `summarize` output, the build identity, the re-measured window stand-in for an 828,000 px² canvas and what canvas size 1440 × 900 gives today, the noise floor per measure with load averages, whether uncapped pacing took effect headless, and two or three sentences on what reads oddly. Then the README (interface including the per-variant builds; how it opens a fixture, drives the orbit, takes each measure and checks settling; what it does not measure, including the earlier observation that on this host the orbit is main-thread-limited so GPU cost under that limit is not visible to the tool; the window stand-in; that it guides the lane and decides nothing about D-72). The owner works under a usage limit: run no extra series, and do not repeat a series unless its run failed.

## Checks at the end

The sealed brief's six: `node --check`; the self-test through the lock, exit 0; after every hold the lock directory absent and nothing listening on 5185; `python3 tools/validation/validate_claims_language.py` from `{REPO_ROOT}`, VALID; `grep -rn` for the home directory's prefix and for `/private/` under `{LANE}/tools` and `{LANE}/probes/T1`, no match; `git status --short` (read-only), only files under your two folders.

## What to return

The sealed brief's return, and in addition: that you verified the five hashes above; every patch you made to the tool, exactly, and why; the self-test result after the last patch; the model you are. Claim no performance, usability or conformance acceptance. End with the line: Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
