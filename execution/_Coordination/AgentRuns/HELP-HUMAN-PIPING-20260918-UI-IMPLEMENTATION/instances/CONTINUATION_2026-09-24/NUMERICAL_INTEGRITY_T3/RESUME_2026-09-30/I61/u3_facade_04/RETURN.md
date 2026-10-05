# I61 RETURN: U3 grant 1d (RV85 T1 and U2)

**Status: complete, with no stop.** Both items are uncommitted in WT/f2a-facade, on top of `886bef131a`.
- **Controls 1–5 hold.** No published byte changes without a permit.
- **Mutants:** all 6 killed, including RV85's W01 and W02 applied verbatim. None is killed by a compile error.
- **Grant 2's added U4 item** (the committed permit-path test aimed at `permitted_run`'s G-B check, RV85 SV18) needs the real permit, so it is not in this grant.

**Run facts.**
- **Role and basis:** TASK Type 2 under ROOT, with no descendants. The grant is ROOT's message and RR "RV85 on U3 grants 1b and 1c: PASS; merged into NUM". The findings' text and W01/W02's patches are from `R/REVIEW_RV85/u3_facade_02/`, read only.
- **Time:** 2026-10-04, 07:38Z to 07:50Z.
- **Host:** the memory guard (PID 5387) was running throughout. Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time, each under a 1200 s alarm.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`.
- **Other agents' files:** I did not touch R/REVIEW_RV85/, R/REVIEW_RV86/, WT/rv85/, WT/rv86/ or I65's files.
- **Writes:**
  - WT/f2a-facade: `lib.rs` (the notice's test-only checks and the test hook seam) and `retained_facade_tests.rs`;
  - WT/scratch/i61_u3_facade_04/: `cand`, `stub`, `mut`, `mutstub`;
  - WT/targets/i61-u3/;
  - this folder.

## 1. T1: a test fails if the notice's `publish` allocates

**`lib.rs` changes. The checks are test-only; production code is unchanged.**
- **`ReservedNotice::publish` checks capacity identity.**
  - It records the diagnostics vector's capacity and the message's capacity before rendering.
  - After the receipt-encoding pushes, it asserts the message capacity is unchanged.
  - After appending, it asserts the diagnostics capacity is unchanged.
  - Any allocation, or growth beyond the reservation, fails the test. This replaces the old `len <= capacity` check, which RV85 showed is always true.
- **`ReservedNotice::reserve`** asserts that the reserved message capacity covers the longest variant.

**`retained_facade_tests.rs`: `u3_r2_notice_bytes_are_pinned` now names the constant.**
- `RECEIPT_ENCODING_DETAIL_MAX` must equal the longest token that `receipt_encoding_detail` returns, taken over all eleven typed `ReceiptCheck` values.
- The reservation must total 196 bytes (135 + 35 + 25 + 1).

**Both RV85 mutants are killed:**
- **W01** (`RECEIPT_ENCODING_DETAIL_MAX = 0`) by the constant pin, and by the capacity identity on the receipt-encoding fallbacks of `u3_each_stage_fault_falls_back_to_the_ordinary_bytes`;
- **W02** (message reservation removed) by the reserve-time capacity check on every W1 start, and by the publish-time identity.

## 2. U2: unfired faults are handed back to the caller after the hop

**`lib.rs`, test builds only.**
- **`carry_test_hooks`** now moves the caller's armed faults in a `Carried` guard.
  - On the worker, the guard installs them.
  - After the work, `hand_back(caller)` sends whatever did not fire back to the caller's thread. A test-only map keyed by the caller's thread id holds them in transit.
  - If the work never runs (a spawn failure drops the closure on the caller's thread), the guard's `Drop` hands the faults straight back.
- **`permitted_dispatch`** calls `retained_tests_hooks::reclaim_handed_back()` right after the hop, which re-arms them on the caller.
- **Production is unchanged:** `carry_test_hooks` is still the identity, and the reclaim is `#[cfg(test)]`.
- **Helpers for assertions:** `armed_names()` and `disarm()`.

**Tests:**
- **The new `u3_unfired_hooks_come_back_across_the_hop`:**
  - Precommit and serializer faults are armed, then preparation refuses on the reserved-stack thread. The faults are in transit, and after the reclaim they are visible to the caller as `["precommit", "serializer"]`.
  - A native fault that does fire there does not come back.
  - On a spawn failure, an armed rebind comes straight back as `["rebind"]`.
- **The custody test (N9/N7)** now also requires the reclaim right after the hop in `permitted_dispatch`.
- **Behind the archive stub, on the actual Direct entry:** a precommit fault armed before a G-B refusal never fires. After the call the caller sees `["precommit"]`, in both modes (RV85's stub row G now shows the leftover).

## Controls

1. **No published byte changes without a permit.**
   - The 324-output fixture sweep is **byte-identical** to base `b54caba7ab` (`faed2518…`).
   - **PP:** 663 outcomes, which is 1c's 662 plus `u3_unfired_hooks_come_back_across_the_hop`. All base outcomes are unchanged, and t13 (Mac) is the only failure.
   - **runner/headless:** 87 outcomes, identical to base.
   - **Production warnings:** equal to base, apart from result_export's own existing warning.
2. **The pinned successor bytes are unchanged** through the private driver and the stub's actual entry (`outputs_sha256.txt`).
3. **Fault controls:** as in 1c. Each fault now also passes the capacity identity.
   - **Stub:** 15 tests pass. All 72 fixture invocations keep their bytes and their report.
   - `u3_capture_permit_is_linear` fails there by construction, because the stub replaces the struct it checks.
4. **Nothing is weakened.**
   - No reader, schema, fixture or existing check changed. The one replaced check was vacuous (`len <= capacity`) and is now strictly stronger.
   - There is no maintained permit constructor.
5. **Mutants: 6 of 6 killed, none by a compile error** (`mutants_summary.txt`).
   - **T1:** W01 and W02, RV85's patches verbatim.
   - **U2:**
     - U2a: the worker does not hand back;
     - U2b: unrun work drops its faults;
     - U2c: the reclaim drops what came back;
     - U2d: the dispatch does not reclaim. This one is killed behind the stub.

## Records (`_run_records/`)

All paths are placeholders, with no machine paths.

**Code and hashes**
- `changed_files_sha256.txt`: against `886bef131a`. Only `lib.rs` and `retained_facade_tests.rs` changed.
- `candidate.diff`, `candidate_status.txt`.

**Suites, warnings and the sweep**
- `suite_{base,final}_{pp,runner}.outcomes`, `build_{base,final}.warnings`.
- `fixture_sweep_compare.txt`.

**Stub and mutants**
- `stub_patch.py`, `stub_test_tail.rs`, `stub_e2e.txt`.
- `mutants.py`, `mutants_summary.txt`.

**Other records**
- `outputs_sha256.txt`, `run_final.sh`.
