# RV109 (RV-P), round 1: independent review of B1's ST slice

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this change.** Build your own oracles; don't rely on the implementer's.

**You hold RV-P for all of B1** (PLAN_v2 §5, RV93's role):
- round 1 reviews ST, which is this brief;
- you confirm ST's repairs;
- round 2 reviews SP, by a later brief;
- you keep the PR-head ledger.

Write your records so a later you can pick them up from the files alone.

## The candidate

- **The branch:** `codex/piping-t3-b1-20261007` at `a8e719f5b4`, in worktree `WT/b1`. It is two commits over main `47a3bdfcf5`:
  - `4a51783e65`: ST;
  - `a8e719f5b4`: test-only.
- **6 files, all in PP `src`:** `lib.rs`, `retained_product.rs`, `retained_memory.rs`, `retained_memory_law_tests.rs`, `retained_facade_tests.rs` and `retained_memory_witness_tests.rs`.
- **The implementer's return:** `R/I85/b1_st_01/RETURN.md` (`f4c1cadc…`). Read it after forming your own view.
- **The specification:** PLAN_v2 §2.1 (`R/I84/b1_plan_01/PLAN_v2.md`, `c85786b7…`), with its §1 fence and RV107's A1 amendments (RR "B1's PLAN_v2 accepted; RV107's A1 amendments; phase 1 dispatched").
- **The contract:** DESIGN_v2 (`R/I78/b0_contract_01/DESIGN_v2.md`): T-4, decisions 1 and 21, `NoTriggeredCase`, and DN §4.3.
- **The witnesses:** I81's PROBE (`R/I81/b1_probe_01/PROBE.md`) §2–§4; the re-basing rulings are in RR "I81's B1-0 probe verified; W-C2's case C established; the re-basing ruled".
- **ROOT's R3 rulings:** RR "R3: I85's ST verified at checkpoint R3; RV-P round 1 dispatched as RV109".

## Review, in priority order

1. **T-4's classifier is right.** Check `case_triggers`, `only_one` and `dn_trigger_excluded` against DESIGN_v2 T-4 and decisions 1 and 21:
   - the verdict is looked up by `basis_ref.ref_id`, never by position;
   - `not_required` exactly when that verdict is `checks_passed`;
   - the exclusion only for Mechanism, Asymmetric or InvalidInput without a W2 publication;
   - everything else in A, including absent entries, `not_assessed` and seedless cases.

   Contest or accept ROOT's uniqueness reading (R3 ruling 1).
2. **Where it runs.** `retained_w1` classifies after coexistence, G-B and `w1_case_id`'s Domain check, and before the notice reservation. With A empty the result is the exact ordinary bytes, no notice, no reservation and no W1 work. Look for any path where W1 work, a notice or a reservation happens for a `not_required` or excluded case, or where a case in A is skipped.
3. **Your own probe on the head, in both modes:** case C alone, two-body B, W6's input and W2b's input, against PROBE §2–§4. Also run I86's `b2_k1e3` (`R/I86/b1_w_probe_01/_run_records/inputs/`), which is W2b's replacement and must stay Sensitive and reach native (in A). Record each outcome, phase and notice count.
4. **c = 1 byte identity.**
   - Every committed c = 1 successor pin is byte-identical between base `47a3bdfcf5` and the head: the milestone, L = 0 and U3's pins, in both modes.
   - Use your own differential of the ordinary and successor bytes over the committed witnesses and fixtures. For every input outside the triggered set, the head's bytes must equal base's.
5. **The seam is behaviour-neutral.**
   - `late_loads_total` and `requested_cases` are written and never read.
   - No adapter event is recorded (RV107 A1-N-2).
   - No receipt byte changes.
   - The overflow path (R3 ruling 2) is noted for SA.
6. **The tests and the re-basings.**
   - W-C1 is on case C, W6 is on case C, and W2b and W6's PHYS-R4 input are `NoTriggeredCase` pins.
   - The four facade oracles take `LOAD_CASES + 1` cases, and I77's renames are applied.
   - Each assertion pins what its name says.
   - No assertion is weakened. Read every removed line.
7. **Mutants.** Write your own, at least the five in PLAN_v2 §2.1, plus any you think the tests miss. Each must be killed by an assertion, not by compilation.
8. **The suites.** Run PP registered and Stale, the runner and the witnesses on base and head, test by test. The only differences should be the listed ones (PROBE §2.4's table, the new pins and the renames). The Mac `t13` failure is known on both sides.
9. **The fence and the guards.** Nothing outside the 6 files, and nothing in FK, the schema, `Cargo.lock`, the reviewed statics, the readers or `grant2.rs`. s11f and the in-fence source-text guards pass, with their expected text unchanged.

## Host

- **Every cargo goes through `WT/tools/t3_cargo.sh`,** the host-wide T3 lock, with `--offline --locked`. Other T3 jobs share the lock (RV108 is reviewing B6 at the same time). Wait for it, and never kill another job.
- **Waits.** Use one wait per job. Every wait loop must also end when the job's process has gone (for example `while kill -0 <pid> 2>/dev/null; do sleep 20; done`), not only on a log line that may never appear. Stop your own waits before you return.
- **Your own copies.** Use a `git archive` of each revision into `WT/rv109/{base,cand}`, with fresh targets under `WT/targets/rv109-*`. Probe code goes in your copies only, under `cfg(test)`.
  - I85's kept scratch and targets (`WT/scratch/i85_b1_st/`, `WT/targets/i85-b1-st*`) are not yours. Don't use or delete them.
- **Not allowed:** DEC-025, evidence sweeps beyond your probe, installs, and Git writes.
- **Scratch** goes in `WT/scratch/rv109_rvp_01/`, and so does `TMPDIR`. Nothing goes to the system temp directory.
- Delete your copies and targets afterwards, but keep your scratch records for your later rounds.

## Output

- **The report:** `R/REVIEW_RV109/rvp_round1_01/REVIEW.md`, with `evidence/` and SHA256SUMS, placeholder paths only. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path, evidence and remedy;
  - a section per item;
  - **the start of the ledger:** each hunk of the head mapped to its reviewed commit.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/rv109_rvp_01/records/` and say so.
- **Budget:** 4–6 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - anything ROOT must rule on.
