# RV113 (RV-R), round 1: independent review of B1's SR-RS slice (the Rust reader)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this change.** Build your own oracles.

**You hold RV-R for B1** (PLAN_v2 §5, RV78's role):
- SR's three lanes: SR-RS now, SR-PY and SR-TS next;
- then SC (corpus 07n).

Keep your records so a later you can continue from the files alone.

## The candidate

- **The branch:** `codex/piping-t3-b1-r-20261007` at `cc81e78801`, in `WT/b1-r`. It is three commits over I1 `262bd687f0`:
  - `d7c76de43f`: the RS alignment;
  - `a4eab1dd01`: B1 tests in `RE/tests/retained_precision_contract.rs`, and its module doc (RV97 R2-N-2);
  - `cc81e78801`: RV95 N-5's direct test, a `#[cfg(test)]` module appended to `RE/src/source_blocks.rs`.

  3 files, +483/−26.
- **The implementer's return:** `R/I90/b1_sr_rs_01/RETURN.md` (`29eb10a3…`). Read it after forming your own view.
- **The specification:** PLAN_v2 §2.4 (`R/I84/b1_plan_01/PLAN_v2.md`, `c85786b7…`), with §1's fence and R8. Also `BRIEFS/B1_SR_RS.md`.
- **The contract:** DESIGN_v2 (`R/I78/b0_contract_01/DESIGN_v2.md`):
  - §2: R-D38 with (4b), and m1–m8;
  - §3.2: text B, P1–P4;
  - §3.3: G8's per-case loop and G5's `not_required` rule.
- **The precedents:** RV78's reader reviews (`R/REVIEW_RV78/`), and I74 decision 9 with RV101 NT-1 and A2-N3 for N-5's test.

## Review, in priority order

1. **Your own cascade census (R5).** Run I1's RS and the head's RS over 07m (294 mutations, 28 must-pass entries), entry by entry, with your own harness. Expected: 0 changes.
2. **R-D38 (4b).** RS admits (4b) exactly where DESIGN §2 does: a native stage that failed with no Run, with the source equality. It refuses m1–m8 and anything else.
   - **D38's audit:** independently list every RS check that assumes a prepared source has a Call or a Run, and compare your list with I90's 18.
   - **Contest or accept I90's choice** to keep ten (4b) conjuncts that a later `PRODUCT_ATTEMPT` check makes redundant at `validate`.
3. **G8 per case.** P1 applies to every case, and P2–P4 hold per case. Dense b = 0 with no parity row is admitted. A parity row on a non-selected case is refused with the right first failure. Check F-1 text B's disclosed limit (P5 deferred).
4. **G5's `not_required` rule:** a null product attempt and the verdict `checks_passed`, with the three conjuncts dropped. A W2-published `not_required` case is admitted.
5. **c = 1 byte identity through precommit.** PP's c = 1 successor pins pass against the head's RS, and the successors equal the fixtures.
6. **The layout stop.** `Validation`, `ValidationError`, `RowClassification` and `AccuracyClass` are layout-unchanged, and there is no visibility change in a D1 `src` item.
7. **N-5's test** is wholly `#[cfg(test)]` and pins what I74 decision 9 and RV101 NT-1 and A2-N3 ask (`failure.block_order` among them).
8. **Mutants.** Write your own beyond I90's 25, especially on (4b), G8's loop and G5. Each must be killed by an assertion, or shown equivalent.
9. **Suites against I1, test by test:** RE, PP, the runner, and the pins.

## Host

- **Every cargo goes through `WT/tools/t3_cargo.sh`** (`--offline --locked`). Other jobs share the lock; never kill another job.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your waits before returning.
- **Writes:** absolute paths for every write. Name no record folder `build`. Record paths are placeholders only, never `~/`, `/Users/`, `/private/` or the worktree's name.
- **Your own copies** under `WT/rv113/`, with fresh targets under `WT/targets/rv113-*`. Delete them afterwards; keep your scratch for SR-PY, SR-TS and SC.
- **Not allowed:** DEC-025, installs and Git writes.
- **Scratch** goes in `WT/scratch/rv113_rvr_01/`.

## Output

- **The report:** `R/REVIEW_RV113/rvr_sr_rs_01/REVIEW.md`, with `evidence/` and SHA256SUMS. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item.
- **Budget:** 4–6 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - anything ROOT must rule on.
