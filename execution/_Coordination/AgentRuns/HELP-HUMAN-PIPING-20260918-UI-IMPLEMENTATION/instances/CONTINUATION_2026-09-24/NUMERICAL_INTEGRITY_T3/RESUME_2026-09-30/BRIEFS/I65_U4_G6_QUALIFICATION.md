# I65: U4 grant 6, qualification and registration

I65 continues, with its existing context, as U4's owner. ROOT is the return path.

## Purpose

Qualify the D1 profile in an actual build, so that ROOT can select M (D-7) and a reviewed source change can register the qualified build. Only then does `admit` grant a permit for an in-domain Direct invocation. Public activation stays with U7.

## Steps

1. **Close the 42 Estimate atoms** (part 2 §8) with in-build values, or with source upper bounds. `profile::ESTIMATES` must reach 0. Re-run the in-build maximum per mode, and re-confirm it is ≤ 0.9 M.
2. **Choose the build identity (or identities) to register,** and say why. Name the profile, opt-level and debug assertions that the milestone run (U3 grant 2 and U7) and U9's gates will use.
   - Every identity gets its own in-build maximum, stack witnesses (W1–W7, the headroom witness) and reviewed-input record.
   - Register nothing that will not be run.
3. **Gate the pinned profile-record test on build identity** (ROOT's ruling on decision 1).
4. **Prepare the qualification record,** `R/I65/u4_g6_01/QUALIFICATION.md`, covering:
   - the in-build E_req,max and E_mov,max per caller and mode;
   - R and the S1 stack evidence, stated explicitly as measured evidence for the inputs and builds run, not a proof. Include carry 8's stated limit and the panic-hook residual;
   - the build identity text and its reviewed inputs;
   - the D1 predicate;
   - the non-claims of D-7: no claim about RSS, allocator overhead, concurrency or supported machines; the reader's process-lifetime statics are counted in every phase.
   
   Propose M under D-7: 4,026,531,840 B, provided E_mov,max + R ≤ M with the 0.9 M margin.
5. **Prepare the registration change. Do not apply it until ROOT says so.** It adds the qualified identity (or identities) to `REGISTERED_PROFILES`, with the threshold M, reviewed inputs and layouts. Put it in `R/I65/u4_g6_01/registration.diff`, together with the test that exercises `admit` granting a permit for the milestone under the registered build.
   - **Decision 7 still holds:** this is not a test permit, but the production profile, registered by reviewed change.
   - **ROOT applies it** after RV89 passes part 2 and an independent reviewer passes G6.

## Host and fence

- **Worktree:** `WT/f2a-memory`, on top of the part-2 commit, uncommitted.
- **Fence:** `retained_memory.rs` (U4's), its law, witness and challenge tests, the FK resource module, and the SR export. `lib.rs` and `retained_product.rs` are not touched.
- **Records:** `R/I65/u4_g6_01/`.
- **Cargo:** the default toolchain, `--locked --offline`, one job at a time. The memory guard must be running. No Git writes. Scratch in `WT/scratch/`.

**Budget: 6 h.** Stop if:
- the in-build maximum exceeds 0.9 M after the Estimates close;
- a witness fails;
- a needed write falls outside the fence.
