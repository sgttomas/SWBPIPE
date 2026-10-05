# I63 (Rust) and I64 (TypeScript): summary-coverage checks in the readers

Two TASKs (Type 2) share this brief, each dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is the return path. Neither delegates.
- **I63** owns the Rust reader.
- **I64** owns the TypeScript reader.

They work in the same worktree, on disjoint paths, at the same time.

## Read first

- `REPO_ROOT/AGENTS.md`, `agents/AGENT_TASK.md`, `projects/chirality-piping/AGENTS.md`, and this brief.
- **The selected design:** `R/I57/summary_coverage_01/ADDENDUM.md`, all of it. Sections 1, 4 and 5 are your specification.
- **Its review:** `R/REVIEW_RV76/summary_coverage_01/REVIEW.md`.
- **The rulings:** in `T3/ROOT_RULINGS_V1.md`, read "Summary-coverage representation selected for the successor" and "I62 checkpoint A: snapshot 04 accepted for reader coverage work".
- **The shared snapshot you test against:** `R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_04.json` and `RETURN.md`. They list every shared file hash, the 47 new mutations with their expected first gate and error code, and the controls that are not yet expressible.
- **Your language's previous author:**
  - I63: `R/I59/rust_reader_01/RETURN.md`;
  - I64: `R/I60/typescript_reader_01/RETURN.md`.
  
  Both are frozen partial work, unaccepted. Their "remaining obligations" lists are out of scope here except the coverage item.

Record the files you actually read, with their sha256, in your RETURN.

## Paths

- `WT` = the T3 worktree root on the M5 host; your dispatch prompt gives the absolute path.
- **Your worktree:** `READER` = `WT/f2a-readers`, branch `codex/piping-f2a-readers-20261003`, head `ae97b7d5c2` (ROOT's unaccepted WIP commit). The working tree also carries I62's uncommitted snapshot-04 edits to the shared schema and corpus. Treat those shared files as **read-only inputs**. Before you start, verify their sha256 against SHARED_SNAPSHOT_04: the schema should be `f943ebd351…` and the corpus `8e333e632c…`.
- `NUM` = `WT/numerics` (records).
- `P` = projects/chirality-piping
- `T3` = P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3
- `R` = T3/RESUME_2026-09-30

## Objective (both readers)

Implement the I57 §4 coverage checks, in the existing gate order and with its exact error codes:
- **G1:** the closed shape. Exactly the required member; null or an array; closed body objects; four booleans; a boolean has_data.
- **G2:** body U encoding.
- **G3:** for a non-null array, the cardinality and ordered unique body ids equal the source inventory associated through this attempt.
- **G5:** first the existing schedule; then the same source/Run/proof binding and the stage and availability implications from §3's table.
- **G5a,** in this order:
  - the existing summary encodings and ranges, and the native p/P/floor rules;
  - the canonical source layout and extent;
  - the compact-flag Boolean feasibility rule (the sixteen A vectors, with D = false for this C3 scope);
  - the estimate/charge rederivation;
  - the exact summary rosters, items 1–4 in §4;
  - the directly derivable has_data constraints.

**Replace your draft's Cartesian or conditional summary-coverage check** with these rules. Never derive private facts from final rows. Earlier original checks still win.

**Eligibility stays closed:** keep the completeness hold that prevents eligibility (`IMPLEMENTATION_COMPLETE` / `SUMMARY_COVERAGE_COMPLETE` or their equivalents stay false). Snapshot 05 and a full review come first.

**The bar,** against snapshot 04:
- all 47 new mutations produce their expected first gate and error code;
- the no-data synthetic case validates with the expected classifications;
- the two complete cases still validate;
- your existing tests still pass.

Report the counts. A mutation whose expected first failure you believe is wrong is a stop: report it, don't adjust it.

Python (I62) implements the same checks at the same time. Where the design leaves a choice, coordinate through ROOT, not by editing each other's files.

## I63: the Rust reader

- **Write fence (READER only):**
  - `P/core/reporting/result_export/src/retained_precision.rs`;
  - `P/core/reporting/result_export/tests/retained_precision_contract.rs`;
  - `P/core/reporting/result_export/src/lib.rs`, for necessary wiring only.
- **The test command,** run from `READER`:
  `cargo test --locked --offline --manifest-path READER/P/core/reporting/result_export/Cargo.toml --test retained_precision_contract -- --test-threads=2`
  with `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `CARGO_NET_OFFLINE=true` and `CARGO_TARGET_DIR=WT/targets/i63-reader/result_export`.
- **Cargo:** one Cargo job at a time, each with a 1,200-second wall (macOS has no `timeout`; use your tool's timeout). I61 runs Cargo in another worktree with its own target; that is allowed.

## I64: the TypeScript reader

- **Write fence (READER only):**
  - `P/apps/desktop/src/features/results/retainedPrecision.ts`;
  - `P/apps/desktop/src/features/results/retainedPrecision.test.ts`.
- **Test commands,** both from `READER/P/apps/desktop`:
  - `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json`;
  - `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`.
- **Runtime:** use the existing `node_modules` link and the existing source-bound WASM assets as they are. No install, no build, and no change to dependencies or configuration.

## Both readers: host, time box, stops

- **Host:** the M5 Max, with the memory guard running (`pgrep -fl memguard`).
- **Never:** new tooling, installs, solver, native, UI or DEC-025 jobs.
- **Time box:** 90 minutes from your first tool call. Stop new edits at 80 minutes, freeze, and report anything unfinished.
- **Stop and report** if:
  - the design and the snapshot conflict;
  - an expected first failure looks wrong;
  - you need a path outside your fence;
  - setup takes more than 5 minutes.

## Output

- **No Git writes and no index operations.** Git reads are fine, with `GIT_OPTIONAL_LOCKS=0`. ROOT commits.
- **Evidence:** I63 writes to `NUM/R/I63/coverage_rust_01/`, and I64 to `NUM/R/I64/coverage_typescript_01/`. Each holds:
  - RETURN.md, short: changed files with sha256; the shared-file hashes verified at start; each command with its result; the per-mutation outcome table (or a reference to a JSON file); deviations; open items;
  - SHA256SUMS.
  
  Put bulk logs in `WT/scratch/i63_coverage_rust_01/` or `WT/scratch/i64_coverage_typescript_01/`, listed with hash, size and path. Use placeholder paths in committed text.
- **End your turn** with a concise status for ROOT: what changed, the counts, anything unfinished, and anything ROOT must rule on.
