# I61: U6e, the reader round (F5, RV79-N1, RV80-N2)

**Read `BRIEFS/U6_FANOUT_COMMON.md` first.** I61 continues with its existing context, and owns U6e through repairs until U4 G5 returns. U3 grant 2 then takes priority; report U6e's state at that boundary.

- **Unit:** PLAN.md §6 U6e. D-U6-1 is already in U6a.
- **Worktree:** `WT/f2a-readers-round`, branch `codex/piping-f2a-readers-round-20261004`, from `844448112f`.
- **Fence:** PY `retained_precision.py` and its contract test; RS `retained_precision.rs` and `tests/retained_precision_contract.rs`; TS `retainedPrecision.ts` and its test; and `fixtures/results/retained_precision_cases.json`, as snapshot **07g**. Disjoint from U6a–U6d.
- **Contents, in order:**
  1. **F5:** amend checkpoint A's D6a so that all three readers enforce decision 2 (A2)'s exact per-case `diagnostic_refs`. That is the diagnostics whose `affected_refs` name the case, once each, in envelope order, excluding `RETAINED_PRECISION_*` and the T1 (a)-omitted disclosure. It kills U1's M09, M10 and M20 at the readers.
  2. **RV79-N1:** an independent D37 expected table, error kind ↔ stage record in both directions. Derive it from the contract and native source, not from any reader's code, and pin it in the corpus. This is a U7 condition.
  3. **RV80-N2:** the `integral_receipt` scope, as RV80 described it in its reader_confirm reports.
  
  Each change carries shared corpus entries that every reader must pass. The corpus moves to snapshot 07g, with the 06d pattern of snapshot files and SHA256SUMS.
- **Not touched:** the completeness flags, which stay false until U7, and RV78-N1, which is deferred to wider F2a.
- **Controls:**
  - all three readers pass 07g in full;
  - the two real milestone receipts still pass, with classes 25/69/3/1 and 25/69/3/2;
  - parity holds on every entry;
  - no 07f entry's outcome changes, except where a ruled change (F5) intends it, each listed;
  - mutants are killed.
- **Records:** `NUM/R/I61/u6e_reader_round_01/`. **Budget:** 8 h.
