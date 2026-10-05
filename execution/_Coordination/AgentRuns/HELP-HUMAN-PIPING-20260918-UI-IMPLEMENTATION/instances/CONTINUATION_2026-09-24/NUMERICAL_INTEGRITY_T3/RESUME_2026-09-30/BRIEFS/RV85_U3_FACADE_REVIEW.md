# RV85: independent review of U3 grant 1 (facade capture, permit branch unreachable)

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path. You do not delegate. **You did not write this code. Don't rely on the implementer's tests or its disposable stub as your oracles.**

## The candidate

- **Commit:** on branch `codex/piping-f2a-facade-20261004`, against base `b54caba7ab` (U1 as merged into NUM). Your dispatch prompt gives the exact head.
- **The change:** I61's U3 grant 1.
  - **PP `lib.rs`:**
    - the dispatch now calls `admit` (G-A), then `ordinary_dispatch` or `permitted_dispatch`;
    - `on_reserved_stack`;
    - `permitted_run`;
    - `retained_w1`;
    - the test hooks.
  - **`retained_memory.rs`:** the consumer shim. `admit`, `LateFacts`, `CompleteFacts` and `PhaseRefusal`, with every permit body statically unreachable.
  - **`retained_product.rs`:** the frozen-candidate split and the late refusal.
  - **`retained_wire.rs`:** `serialize_frozen`, and R08 pinned at its call site.
  - **Tests:** `retained_facade_tests.rs` (new) and `retained_wire_tests.rs`.
  - **`PP/Cargo.toml`:** `result_export` moves to a runtime dependency (decision 5).
  - **Six downstream `Cargo.lock` files:** one dependency edge each, plus the in-repo `result_export` package block where it was missing. ROOT applied these under ruling R-3.
- **The implementer's account:** `R/I61/u3_facade_01/CHECKPOINT.md` and `RETURN.md`, with `_run_records/`.
- **The rulings:** in `T3/ROOT_RULINGS_V1.md`, "U3 grant 1 verified; R-1, R-2 and R-3 ruled". Also D-2, D-3 = S1, D-5, decision 5, decision 7, RR:8436, and the U1 rulings.

## Review, in priority order

1. **No published byte changes while no permit exists.**
   - With `RegisteredProfile` uninhabited, confirm by reading that the permit branch is statically unreachable. Confirm that `admit`'s report equals the old `assess` report byte for byte.
   - Run your own fixture sweep: at least 20 request fixtures × 5 routes (typed default, typed mode, value, retained Direct, retained Headless) × 2 modes, compared with base `b54caba7ab`. Include the admission report.
   - Run PP `--lib` and the PP, runner/headless and result_export integration suites, and compare their failure sets with base. The expected Mac platform failures are exactly t13 and the two load_reference tests.
2. **The permitted path's structure, by reading.** It cannot be exercised without a permit.
   - `on_reserved_stack`: spawn-failure fallback, panic re-raise, and that the request is not lost.
   - `permitted_run` and `retained_w1`:
     - the single observed ordinary run;
     - that the ordinary envelope is never mutated and is returned untouched on every fallback;
     - that no fallible step follows the transfer's first move;
     - that coexistence and the late and complete gates are in the right order (I51 COMPOSITION §2);
     - that precommit validation uses the actual invocation.
   - Check the precommit failure route: a reader refusal becomes the ordinary fallback.
3. **The frozen-candidate split (`retained_product.rs`)** against I51's design. Check that staging applies to a copy, that `into_ordinary` returns the untouched owner, and that the F-4 unwrap removal is correct.
4. **Single-parse custody (A-1, RV82-N9)** and **R08 at its call site (A-2, RV82-N1′).** Rerun RV82's R08b mutant verbatim.
5. **Two design points ROOT flagged.**
   - **`CapturePermit` now derives `Clone` and `Copy`.** Can a permit then outlive or be reused across invocations? Should it be linear (not `Copy`), consumed by `permitted_dispatch`?
   - **The test hooks are `thread_local!`.** Once grant 2 makes the permitted path run on the reserved-stack thread, a hook set on the caller's thread would not fire there. Say whether the committed fault tests could then pass vacuously, and what grant 2 must do.
6. **The lock deltas (R-3).** Each of the six must add only the in-repo `open_pipe_stress_result_export` edge or package block, with no registry package or version change. Run `cargo metadata --locked --offline` for each manifest you can; the Tauri app may need network for an uncached registry crate, and you should say so.
7. **Mutants.** Re-run I61's committed-test mutants, and add at least four of your own on the dispatch, the fallback and the frozen split. Report survivors.
8. **Nothing weakened:** no reader, schema, fixture or check changed; no maintained permit constructor; no test permit in maintained code.

## Host and method

- **Your copy:** build from `git archive` of the candidate in `WT/rv85/`, with targets `WT/targets/rv85/`. Keep logs in `WT/scratch/rv85_u3_facade_01/`, and delete the copy afterwards.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time.
- **The memory guard** must be running.
- **Never:** Git writes, index operations, installs, new tooling, or solver-at-scale, DEC-025 or native jobs. Nothing goes to the system temp directory.
- **Other TASKs are working;** don't touch their files. They are I65 on U4 G4 records, RV83, RV84, and I61, which may be idle or working on grant 2.

## Output

- **The report:** `NUM/R/REVIEW_RV85/u3_facade_01/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - a short section per review item.
  
  Include a SHA256SUMS. Use placeholder paths only.
- **Time box:** 3 hours.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
