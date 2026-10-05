# RV93: independent review of U3 grant 2 (the permitted path and the live milestone)

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path. You do not delegate. **You did not write this code. Don't rely on the implementer's tests as your oracles: build your own for every claim that matters.**

## The candidate

- **Commit:** I61's U3 grant 2, committed by ROOT on `codex/piping-f2a-memory-20261004` on top of the registration commit `0c7827b6ad`. Your dispatch prompt gives the exact head.
- **The registered build** is the one identity in `REGISTERED_PROFILES`, with M = 4,026,531,840 B:
  - aarch64-apple-darwin, rustc 1.97.1 `8bab26f4f68e`;
  - debug, opt-level 0, debug assertions on, panic=unwind, no RUSTFLAGS.
  
  This host compiles exactly that build in a default `cargo test`. Check it first: the build script's `OPS_RETAINED_BUILD_IDENTITY` and `OPS_RETAINED_REVIEWED_INPUTS` must equal the registered strings.
- **The implementer's account:** `R/I61/u3_grant2_01/RETURN.md` and `_run_records/`.
- **The brief:** `BRIEFS/I61_U3_GRANT2.md`.
- **The rulings** in `T3/ROOT_RULINGS_V1.md`:
  - U3 grants 1 to 1d, R-1 to R-3, and RV85's findings (U1, U4, N2, N3);
  - "RV86 on U5: PASS, with stated limits…";
  - D-U6-5;
  - the U4 G6 and registration rulings;
  - RR:8436 (one-case unavailable has no successor);
  - ROUTING:98 (rejected_stress_range).

## Review, in priority order

1. **The milestone, through the actual Direct entry, in both modes.**
   - Run RF-SKEW-T-CANT-OFF-122-r1e-04 yourself through `run_linear_static_preview_value_with_retained_direct`. Check that it publishes a `Successor` whose bytes equal U1's pinned files (sparse `ac6986b0…`, dense `6cd1d249…`).
   - Run the published bytes through the three F2a readers (Python, Rust and TypeScript, as the candidate carries them). Report each verdict.
   - Rerun U5's comparison (`u5_compare.py`, with RV86's committed 7,240-byte extract) on the bytes you obtained. Check the result against RV86's stated limits.
2. **Refusal and coexistence controls on the permitted path.**
   - **Every fallback** (Preparation, Native, Candidate, Serializer, Precommit and Staging) gives the ordinary bytes plus exactly one N1 notice.
   - **Every no-W1 refusal** gives exact ordinary bytes and no notice: G-A, G-B, G-C including `OrdinarySolveNotAttempted`, stack, domain and coexistence.
   - Build at least one of each class yourself, from inputs or fault hooks, rather than reading the implementer's table.
   - **RV85 item 5:** confirm the fault hooks actually fire on the reserved-stack thread. A mutant that breaks hook propagation must fail a committed test, so no fault test can pass vacuously.
3. **One ordinary run (B-1 / S-7), and B′.**
   - The permitted path runs exactly one ordinary run per invocation. The no-permit path makes no extra ordinary run and no extra clone.
   - Check that the dispatch-count hook cannot change a published byte and is unreachable from production use, or explain how it is gated.
4. **SV18 (RV85 U4/N2).** Remove `permitted_run`'s G-B check. A committed permitted-path test must fail, and not by a compile error.
5. **Controls outside the permitted entry.**
   - **In the registered build,** every other route is byte-identical to base `0c7827b6ad`: typed default, typed mode, value, retained Headless, the runner, and Direct on out-of-D1 inputs. Run your own fixture sweep: at least 20 fixtures × 5 routes × 2 modes, with the admission report.
   - **In a non-registered build:** use a separate target dir with an identity input changed, such as a non-empty RUSTFLAGS, so the build is `Stale`. There, the Direct entry is also byte-identical to base.
   - **No runner-workspace test is granted a permit** (RR "RV89 passes the pre-registration delta…"). Probe it in your own copy only.
6. **RV85 U1 (optional in the grant):** if I61 bound the permit to its invocation, check that a permit cannot be reused or outlive its invocation. If not, say whether the remaining risk is real in the registered build.
7. **Mutants.** Rerun I61's committed-test mutants, and add at least five of your own:
   - on the successor-or-ordinary choice;
   - on the N1 notice count;
   - on the fallback that returns the untouched ordinary envelope;
   - on the dispatch count;
   - on the G-C unattempted-solve decline.
   
   Report survivors.
8. **Nothing weakened:**
   - no reader, schema, fixture or existing check changed;
   - no test permit, and no maintained permit constructor outside `admit`;
   - `retained_memory.rs` unchanged by the grant.
   
   Read the full diff against `0c7827b6ad`.

## Host and method

- **Your copy:** build from `git archive` of the candidate in `WT/rv93/`, with targets `WT/targets/rv93/`, a separate one for the Stale build, and logs in `WT/scratch/rv93_u3_grant2_01/`. Delete the copies afterwards.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time.
- **Python:** I52's CLIs and RV86's extract for U5. For TypeScript, link the existing `node_modules`, and use prebuilt WASM only if the candidate already carries it.
- **The memory guard** (PID 5387) must be running.
- **Never:** Git writes, index operations, installs, new tooling, or solver-at-scale, DEC-025 or native jobs. Nothing goes to the system temp directory.
- **Other TASKs are working.** Don't touch their files. They are I61, whose follow-on may run in WT/f2a-memory, and I65 (U4 G7 in WT/scratch/i65_u4_g7_01).

## Output

- **The report:** `NUM/R/REVIEW_RV93/u3_grant2_01/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - a short section per review item.
  
  Include a SHA256SUMS. Use placeholder paths only.
- **Time box:** 4 hours.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
