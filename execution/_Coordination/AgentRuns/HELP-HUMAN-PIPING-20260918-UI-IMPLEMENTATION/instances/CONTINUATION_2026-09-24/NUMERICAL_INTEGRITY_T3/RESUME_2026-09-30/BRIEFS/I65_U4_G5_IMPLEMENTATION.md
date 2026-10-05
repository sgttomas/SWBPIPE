# I65: U4 grant 5, the memory profile and admission law in code

I65 continues as a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), with its existing context. ROOT is the return path, and I65 does not delegate. **I65 owns U4 through repairs.** This is U4's implementation grant, PLAN.md's G5, under the derivations of G2–G4 as ruled.

## Purpose

Write the admission law that makes a permit possible, while keeping it **unreachable in maintained code until G6 registers a qualified build**. G6 then evaluates the profile in the actual build, records the stack evidence, and ROOT selects M (D-7). After G5, the only thing between the milestone and a live permit is G6's registration: a reviewed source change that adds the registered identity text and constants.

## Basis

- **The derivations:** `R/I65/u4_g2_01/`, `u4_g3_01/` and `u4_g4_01/`, including `ADDENDUM_L128.md`, with every ROOT ruling from "U4 plan: decisions…" through "U4 G4 addendum at l ≤ 128". Especially:
  - D1 = D1.0–D1.11, with `l ≤ 128`;
  - D-3 = S1;
  - D-6 = (a), as amended by B-3/S-5 and extended to the reader statics;
  - S-1;
  - the admission law restated (S-5);
  - the margin rule.
- **The interface:** `API_G4.md`, plus U3 grant 1c's `admit` signature, `Result<(CapturePermit, RetainedAdmissionReport), RetainedAdmissionReport>`. Record that signature in your RETURN as the API.md correction.
- **Your carry list:** `NOTES_G4.md` §5, items 1–11, plus RV85's U1 (bind the permit to its invocation; check linearity structurally), which is optional.
- **Reviews in flight:** RV87 is reviewing G4, and RV83 and RV84 are confirming their repairs. Their findings reach you through ROOT. If one changes a number, the number moves into G6's constants; G5's structure should not depend on it.

## Where to work

- **Worktree:** `WT/f2a-memory`, branch `codex/piping-f2a-memory-20261004`, from the facade branch head `8abb5274a9`. That head holds U1, U2 and U3 grants 1–1d.
- **Records:** `NUM/R/I65/u4_g5_01/`. **Targets:** `WT/targets/i65-g5/`.
- **Write fence:**
  - `PP/src/retained_memory.rs`, which is U4's (D-5);
  - a new `PP/build.rs` and `PP/src/build_identity.rs`, for D-6;
  - `PP/Cargo.toml`, for the `build` key only;
  - a new FK resource module, plus its `mod` line, exporting the kernel's cap-bounds;
  - an SR stride export, only if T14 needs one;
  - new PP test files, plus `retained_facade_tests.rs` where the shim's tests move;
  - **a narrow D-5 exception, ruled by ROOT:** exactly the two struct-construction sites API_G4.md §3 names, at `retained_product.rs` (`LateFacts { …, capture }` in `prepared_case_source`) and `PP/lib.rs` (`CompleteFacts { …, capture }` in `permitted_run`). No other line of `lib.rs` or `retained_product.rs`.
  
  I61 stays the integration owner for everything else there. The stack thread placement is already in U3.

## What G5 delivers

1. **G-A `admit`.** It runs:
   - the census, extended with T03's nested typed walk, the typed capacity caps (S-3), `max_string_bytes`, `max_key_bytes`, D1.11's `control_bytes`, and D1.10's provenance first-byte test, all allocation-free;
   - the D1 predicate, D1.0–D1.11. Headless is refused (D-2, U3 F-5);
   - the build status (D-6): Missing or Stale fails closed;
   - the cap-priced admission bound against M.
   
   Every refusal preserves all facts and the first failing clause, with its `unavailable_precondition` kind.
2. **`RegisteredProfile`,** with the cap-priced constants as named, reviewable expressions over in-build `size_of`/`align_of` and the source-derived upper bounds (T01 and T10). **The registered-identity list stays empty, so no profile, and no permit, can be constructed in maintained code or tests until G6** (decision 7).
3. **`check_late` and `check_complete`,** with `PhaseFact` and `PhaseRefusal { gate, fact, observed, cap }` (API_G4.md §2), reading the gate facts without allocating; and **`budgets()`**, with U3's budgets B-1 to B-10.
4. **D-6:**
   - `build.rs` writes the one-line escaped identity, with `rerun-if-changed` on `build_identity.rs` and the toolchain variables. An empty variable is a value; a read failure is `unavailable`.
   - `option_env!` is used, with absence meaning `Stale`.
   - Compile-time layout witnesses cover the std and serde_json types and the reader types T17 prices.
   - The reviewed-lock record binds the PP lock hash and the reader's 13 `include_str!` hashes. **Never a compile error;** a mismatch means `Stale`.
5. **S1, the stack:** the constant R = 64 MiB, its `cfg(test)` override, the `StackReservation` refusal, and the witness tests W1–W7 at R/16 (STACK_PLAN.md and G3's inventory). They run through the private driver where a permit is needed.
6. **Tests:**
   - exhaustive law checks over the cap ranges;
   - cap and cap+1 for every D1 fact;
   - unknown, stale, overflow and partial refusals, with the ordinary bytes identical;
   - M−1, M and M+1 boundaries on the bound arithmetic;
   - the D1.10 and D1.11 census readers;
   - the build-identity encoder round-trip, with every key present in order;
   - an allocation challenge in an isolated test binary, using the existing peak-tracking test allocator. It is a challenge only, not a proof or production guard.
7. **Carry items 3, 8 and 10** from NOTES_G4.md §5, as listed there.

## Controls (each failure is a stop)

1. **No published byte changes while no profile is registered.** Every route and fixture, in both modes, produces bytes identical to base `8abb5274a9`, including the admission report. A new report field is allowed only if it is private, not serialized.
2. **Nothing is weakened:**
   - no reader, schema, fixture or existing check changed;
   - no permit constructor reachable without a registered profile;
   - no test permit in maintained code.
3. **Mutants** for every D1 clause, every gate, the bound comparison, the build-identity check and the stack refusal. None may be killed only by a compile error.

## Host

- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time. I66 and possibly I61 also run cargo, in their own targets.
- **Scratch** in `WT/scratch/i65_u4_g5_01/`, never the system temp directory. The memory guard must be running.
- **Never:** Git writes, installs, new tooling, or native, solver-at-scale or DEC-025 jobs.

## Budget and return

**Budget: 14 h.** If the diff passes about 1,500 lines, or a natural boundary appears (for example D-6 and the census done, the profile and gates still to do), return the completed part for review and continue on ROOT's word. Stop if:
- a contract or derivation reading is needed beyond the rulings;
- an ordinary byte changes;
- a check would be weakened;
- a write falls outside the fence.
