# I61: U3, facade capture installation (grant 1)

I61 continues as a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), with its existing context. ROOT is the return path, and I61 does not delegate. **I61 owns U3 through repairs,** and is the single integration owner for `PP/lib.rs` (D-5).

## Purpose

U3 connects U1's serializer to the actual facade, so that a permitted Direct invocation of the milestone publishes its successor from the facade's single ordinary run. Without a permit, behaviour stays exactly today's.

## Basis

- **The plan:** `R/I61/step4_plan_01/PLAN.md` §2, U3.
- **Experiment 03's design inputs D-a to D-d:** `R/I61/receipt_experiment_03/RETURN.md`, "Design inputs for U3".
- **The U3/U4 interface:** `R/I65/u4_g2_01/API.md`, which RV83 found consistent. The refusal map is `DOMAIN.md` §3.
- **The rulings** in `T3/ROOT_RULINGS_V1.md`:
  - D-2 (Direct only);
  - D-5 (U4 owns `retained_memory.rs`; U3 owns the dispatch);
  - decision 5 (precommit Rust reader);
  - decision 7 (no test permit in maintained code);
  - D-3 = S1 (the reserved-stack thread);
  - D-4 and D-4b;
  - RR:8436 (one-case unavailable has no successor);
  - design-to-budget (G4 sets U3's budgets);
  - the U1 rulings, including RV82's N3 (B′ is U3's).
- **The code:** U1 at `b54caba7ab`. RV82 is reviewing it; any repair it requires reaches this branch by merge.

## Where to work

- **Worktree:** `WT/f2a-facade`, branch `codex/piping-f2a-facade-20261004`, from `b54caba7ab`. **Records:** `NUM/R/I61/u3_facade_01/`. **Targets:** `WT/targets/i61-u3/`.
- **Write fence:**
  - `PP/lib.rs`: the retained dispatch and `run_linear_static_preview_captured`/`_observed`;
  - `PP/retained_product.rs`: the frozen-candidate split and the successor carrier;
  - `PP/retained_wire.rs`, only where U3 consumes it;
  - `PP/retained_memory.rs`: the permit consumer only. U4 owns the rest, and no profile or permit is constructed;
  - `PP/Cargo.toml` and `Cargo.lock`: `result_export` moves to a runtime path dependency (decision 5);
  - PP tests.
- **Not in the fence:** runner/headless (D-2), the readers, the schema and the fixtures.

## Two phases, because the permit does not exist yet

Until U4 G5 adds `admit` and a registered profile for D1, no maintained code can construct a `CapturePermit`, and decision 7 forbids a test permit.

**Grant 1, now:** everything that does not need a live permit.
- the frozen-candidate split (I51's design), owning the fallback copy (D-b);
- the successor carrier in the retained output (D-a);
- precommit validation through the Rust reader, with the receipt-failure route;
- the transfer, with no fallible allocation after the first mutation (T18);
- the W1-unavailable fallback with preserved ordinary bytes;
- the dispatch structure that will call `admit`, `check_late` and `check_complete`, and run on the reserved-stack thread. It uses the interface in API.md, with the permit branch unreachable.

Exercise the permitted path through the private driver in committed tests, as U1 does. Also run it end to end once in a disposable archive behind a stub, as experiment 03 did. The archive run is evidence, not code.

**Grant 2, after U4 G5:** connect the real `admit`, then commit the facade-level permit-path tests, B′ and the byte controls on the actual entry.

## First, a short design checkpoint

**Return early only if a ruling is needed.** Put proposals with citations in `R/I61/u3_facade_01/CHECKPOINT.md`, at most about 90 minutes in, for these:
1. **The successor carrier's type and visibility** (D-a), and whether any public type changes.
2. **The public notice.** Does a permitted invocation that ends W1-unavailable, or a permit refusal, carry `RETAINED_PRECISION_UNAVAILABLE` under C1 §2 (C1:64, 68; I30:98), and with what bytes? Today a refused retained entry returns exactly the ordinary bytes. Any change to a published byte is a ruling, not an implementation choice.
3. **Where precommit validation sits** and its failure route. This is how a reader refusal becomes the ordinary fallback.
4. **The test strategy under decision 7.**

**Proceed meanwhile** with the parts that need no ruling. If no ruling is needed, don't stop: record your choices in RETURN.md.

## Controls (each failure is a stop)

1. Every existing route, the facade without a permit, and both retained entries, produce byte-identical output to base for every fixture, in both modes.
2. The private-driver permitted path publishes U1's pinned successor bytes for the milestone, in both modes.
3. Fault controls at each new stage: preparation, native, proof, serializer, precommit validation and transfer. Each falls back to the preserved ordinary bytes, or to the ruled notice.
4. Nothing is weakened: no reader, schema, fixture or existing check, and no maintained permit constructor.
5. Mutants for each new branch, none killed only by a compile error.

## Host

- Default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time.
- The memory guard must be running.
- **Never:** Git writes or index operations, installs, new tooling, or native, solver-at-scale or DEC-025 jobs.
- RV82 is reviewing U1 from its own archive, and I65 is on U4 G3 records; don't touch their files.

## Budget and return

Budget: 8 h, plus the checkpoint if needed. Return once, with RETURN.md (changed-file hashes, controls, mutants, findings), SHA256SUMS and a concise status. **RV82's findings on U1 grant 2** come to you through ROOT, and are repaired in WT/f2a-serializer as a separate item.
