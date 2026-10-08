# B1-SR-RS: the Rust reader (R-D38, F-1 text B, G8 per case, G5's `not_required` rule)

Read `R/BRIEFS/B1_COMMON.md` first; its rules hold.

**The specification** is PLAN_v2 §2.4: the common part and SR-RS. It applies with §1's fence for SR-RS:
- RS (`RE/src/retained_precision.rs`) and `RE/tests/retained_precision_contract.rs`;
- `RE/src/source_blocks.rs`, for RV95 N-5's `#[cfg(test)]` test only.

R8's stops apply. **RS is on the D1 call graph,** because PP compiles it for precommit. A change to the layout of `Validation`, `ValidationError`, `RowClassification` or `AccuracyClass` is a stop.

**The contract:** DESIGN_v2 (`R/I78/b0_contract_01/DESIGN_v2.md`):
- §2: R-D38 with (4b), and m1–m8;
- §3.2: text B, P1–P4 (P5 deferred);
- §3.3: G8's per-case loop with `RETAINED_PRECISION_PREPARATION_MISMATCH`, and G5's `not_required` rule.

**Your worktree:** `WT/b1-r`, branch `codex/piping-t3-b1-r-20261007`, cut by ROOT from I1 (`262bd687f0`), which carries B6's 07m. You are I-RS. SP (`WT/b1`) and SA (`WT/b1-a`) run beside you; touch none of their files.

## First: the cascade census (R5)

Run the aligned reader over 07m's 294 mutations and 28 must-pass entries.
- **Expected: zero changes.**
- **Any change stops the work.** Return to ROOT with the census, under ruling point R5.

## Then

- **G8** is widened to every case, with P2–P4.
- **G5** drops the three conjuncts: `initial.kind == report`, `outcome == checks_passed`, `w2 == not_triggered`.
- **D38's obligation** `[r1: N-6]`:
  - list every check in RS that assumes a prepared source has a Call or a Run;
  - relax each to (4b), or show it does not apply.

  The list goes in your RETURN; RV-R checks all three readers' lists.
- **Unit tests** on synthetic n-case receipts.
- **The module doc** of `RE/tests/retained_precision_contract.rs` is reworded (RV97 R2-N-2).
- **RV95 N-5's direct `#[cfg(test)]` test** in `RE/src/source_blocks.rs`, as I74 decision 9, RV101 NT-1 and RV101 A2-N3 specify (`failure.block_order` in N-5's direct test).
- **The RV108 notes routed to B1's other lanes are not yours.** N1 and N2 are SR-PY's; N4 and N6 are SR-TS's; N3 and N5 are SC's.

## Acceptance

- The census holds (zero changes).
- RS passes 07m in full; every count change is an added test.
- G8 and G5 behave per DESIGN §3.3 on your synthetic receipts.
- D38's list is complete.
- c = 1 byte identity holds through precommit: run PP's c = 1 successor pins against RS at your head.
- The source-text guards that read RE pass, notably RE's carrier test.
- **Mutants, killed by assertions:**
  - G8's per-case loop (case 0 only);
  - each of G5's three dropped conjuncts restored;
  - each relaxed D38 check restored;
  - the P2–P4 checks.

**Return** with:
- the head and commits;
- the census;
- D38's list;
- the evidence per item;
- the suite differences against I1;
- the mutants;
- anything for ROOT.

RV-R (a fresh reviewer) follows, then I3.

**Records:** `R/<id>/b1_sr_rs_01/`.

**Budget:** 7–10 h.
