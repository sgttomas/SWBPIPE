# I13 (F1b) gate pause state (2026-09-28, 18:50Z, ROOT's pause order: low battery)

All gate processes were killed at 18:50:34Z: the part-2 driver, the running candidate probe, and the host watchers and waiters. No cargo was running. No Git writes were made.

- Candidate: `948e0bb99`.
- Records: `<wt>/scratch/i13/gate/`.

## Finished and complete

- **Probe build.**
  - The candidate full-envelope variant (`main.rs` `cd1052f7…`, `Cargo.lock` `d7bdd546…`, the lock unchanged by the build) was built from `git archive 948e0bb99` in `gate/tree/`.
  - Target: `<wt>/gate-cand-full-target`. Binary sha256 `eedb7026da4addf819b5fea7a028b7f69b1322e42ddbd6557c925349c2f07672` (copy in `gate/build/`).
- **Part 1: DONE, all 884 runs.**
  - Driver: G1's `gate_run_base_full.py` (`8cf952ae…`), unchanged, with 4 workers. Phase A (the 24 runs at 10,000 members) ran 2 at a time and alone.
  - Time: 17:28:08Z–17:33:19Z.
  - Records in `gate/part1/`: `runs.jsonl` sha256 **`954300267e4bdc8d1576a9c1ab16c8a443324567c66f26b304f58ac755ff7b7b`**; base (G1 full) `9139140c80a6b5233ef67c01b8eaec0df85e7c4ed44df41249cbb73ff902a38d`.
  - `gate_check`: **PASS**. 764 evaluated, 332 trusted, 0 trusted breaches, 0 violations (`part1/result_part1_candidate.json`, `part1/gate_check.log`).
  - Comparison (`gate/compare_gate_f1b.py` → `gate/compare_part1.json`):
    - **C3:** 832 runs, **0 differences** (outcome, ok, exit, full-envelope sha256, summary sha256, error text, gate_check standing).
    - **C1:** 28 changed, all within the ruled 28-run list; 0 changed outside C1 and C2.
    - **C2 dense:** 12 of 12 are the guard's refusal before any n² allocation (rss 0.23–0.61 GB).
    - **C2 sparse:** 8 of 12 complete, with named M03 refusals (`NUMERICAL_INTEGRITY_UNRESOLVED`).
  - **Finding (the part-1 verdict is FAIL on this criterion alone):** 4 of 12 C2 sparse runs abort at the heap cap. These are RF-LARGE-CONT-n10000-AX and -ROT, both entries.
    - The allocation is 8,589,934,592 bytes.
    - The diagnostic backtrace (`gate/diag/`) puts it in PP `solve_preview_reduced_system` → sparse_direct `solve_symmetric_system_from_entries` → `SymmetricProfileMatrix::from_entries` (identity order). This is main's legacy DEC-050/053 observation lane, and F1b did not change it.
    - The lane builds the original-order profile only to report `original_profile_entry_count` and `original_max_half_bandwidth`.
    - For CONT n10000 that profile is 675,179,982 entries (5.4 GB). Vec doubling asks for 8 GiB.
    - On main the same runs aborted earlier, at the dense K (480,048-byte allocation).
- **RV11D-N2 on the 13 cases of 1,000 or more members (typed, sparse, observation build of `948e0bb99`): DONE** (`gate/rv11d_large/`, `runs.jsonl` sha256 `2a035a11…`).
  - The 6 n01000 cases: all equal (6 or 1,506 reactions and 1,000 end-action sets each), 0 refused, 0 mismatches.
  - Not completed:
    - RF-MECH-LINE-IN-CHAIN1000: a mechanism, nothing published;
    - CHAIN and TREE n10000 (4 cases): M03 refusal, nothing published at b = 0;
    - CONT n10000 (2 cases): the same heap-cap abort.
- **Part 2: 1 of 8 runs complete.**
  - Base RF-LARGE-CHAIN-n01000-ROT dense captured timed out at 1800.04 s (exit −9). It is in `gate/part2/base/runs.jsonl` (1 line, sha256 `e6d2bf67…`).
  - Load: 4.25 before, 5.03 after.

## Void (interrupted)

- **Part 2, candidate RF-LARGE-CHAIN-n01000-ROT dense captured:** started 18:17:00Z and killed at about 18:50Z. There is no record line; `gate/part2/cand/` holds only empty directories.

## To re-run on resume

1. **Part 2, all four pairs, interleaved base then candidate, one at a time, at 1800 s**, with `gate/gate_part2.py`, a fresh output directory, `<wt>/gate-base-full-target` (sha256 `577b10d4…`) and `<wt>/gate-cand-full-target` (sha256 `eedb7026…`).
   - The completed base run above may be kept, or re-run for strict interleaving; ROOT decides.
   - At minimum, re-run the void candidate run and the remaining 6 runs.
2. **Nothing else.** Part 1, its comparison and RV11D-N2 are complete.
   - The PASS verdict waits on ROOT's ruling on the CONT n10000 sparse heap-cap finding, and on part 2.
