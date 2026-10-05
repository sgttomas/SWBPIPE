# I61: U3 grant 2, the permitted path in maintained tests, and the live milestone

I61 continues as U3's owner and as the `PP/lib.rs` integration owner (D-5), with its existing context. ROOT is the return path. **This grant starts only after ROOT has applied the registration** (U4 G6's `registration.diff`, after RV89's and RV87's review), so that a permit is constructible for in-domain Direct invocations in the registered dev/test build.

## Basis

- **U3 grants 1 to 1d** (`R/I61/u3_facade_01` to `_04`) and their reviews (RV85).
- **U4 G5, G6 and the registration** (`R/I65/u4_g5_01`, `u4_g6_01`), with `API_G4.md` and the rulings through "U4 G6 committed unregistered…" and the registration ruling.
- **The U5 rulings:** "RV86 on U5: PASS, with stated limits…", which defines the milestone's reference claim and its limits.
- **D-U6-5:** U6's carrier fixtures are byte-identical copies of PP's pinned successors.

## Where to work

- **Worktree:** `WT/f2a-memory`, on top of the registration commit. That branch holds U1, U2, U3 grants 1–1d, U4 G5 and G6, and the registration. Leave the work uncommitted.
- **Records:** `NUM/R/I61/u3_grant2_01/`.
- **Fence:** the PP dispatch and facade (`lib.rs`), `retained_product.rs` where U3 owns it, PP tests, and U5's comparison script (`R/I61/u5_reference_01/_run_records/u5_compare.py`, for the extract pin only). `retained_memory.rs` is U4's: read it, and don't edit it.

## Deliverables

1. **Committed permit-path tests on the actual Direct entry,** in the registered build:
   - the milestone publishes a `Successor` in both modes, with bytes equal to U1's pinned files (sparse `ac6986b0…`, dense `6cd1d249…`);
   - every fallback (Preparation, Native, Candidate, Serializer, Precommit and Staging) gives the ordinary bytes plus exactly one N1 notice;
   - every no-W1 refusal (G-A, G-B, G-C including `OrdinarySolveNotAttempted`, stack, domain, coexistence) gives exact ordinary bytes.
2. **B′ (RV85 N3):** the no-permit path runs no extra ordinary run and makes no extra clone.
3. **B-1 / S-7:** exactly one ordinary run per invocation on the permitted path, pinned by a test that counts dispatches. Add the `lib.rs` hook this needs.
4. **RV85 U4 and N2:** a committed permitted-path test that kills SV18 (`permitted_run`'s G-B check removed).
5. **D-U6-5:** one PP assertion that `include_str!`s U6's two carrier fixtures and compares them with the live serializer output.
6. **U5 on the committed facade output.**
   - Switch `u5_compare.py` to RV86's committed 7,240-byte extract (the two-line pin), keeping the hash assertion on I50's full log.
   - Rerun it unchanged otherwise, on the successor bytes the actual Direct entry publishes in the registered build, in both modes.
   - Record the result against RV86's stated limits.
7. **RV85 U1 (optional):** bind the permit to its invocation, if it is cheap now that `lib.rs` is in scope.

## Controls (each failure is a stop)

1. **Builds without a registration** (any other identity, or the unregistered build) keep every existing route byte-identical to base: your 324-output sweep.
2. **In the registered build, every route except the permitted Direct entry is byte-identical to base.** The Direct entry differs only as the rulings define: the successor on success; the N1 notice on a W1-ran fallback; exact bytes otherwise.
3. **Nothing weakened:** no reader, schema, fixture or existing check changed, and no test permit.
4. **Mutants** for every new test, none killed only by a compile error.

## Host

- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time.
- **Python:** for U5, use RV86's committed extract and I52's CLIs.
- **The memory guard** must be running.
- **Never:** Git writes, installs, or native, solver-at-scale or DEC-025 jobs. Nothing goes to the system temp directory.

## Budget and return

Budget: 6 h. Return once, with RETURN.md, SHA256SUMS and a concise status.
