# RV107: independent review of B1's implementation plan (documents only)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this plan.** ROOT rules on the plan (its ruling point R1) only after your review. B1 is the largest unit left before PR-B1, at 67–102 h agent time, so an error here is expensive.

## The candidate

- **The plan:** `R/I84/b1_plan_01/PLAN.md` (sha256 `7f9699f34de3a64a587d3e3e5f662b899d3e86ba2d9176b647df624012c4bb5c`), with its `_run_records/`.
- **The brief it answered:** `R/BRIEFS/B1_PLAN.md`. The three later basis changes are recorded in RR:
  - "Owner decision: memory up to 64 GiB is ROOT's; B1's target reopened";
  - "Owner decision: M's practical limit is 12 GiB; target machines";
  - "I82's addendum: B1's target is S3, one tier at D1's caps with C = 3".

## Its basis

- **The contract:** `R/I78/b0_contract_01/DESIGN_v2.md`, with RR "B0 selected on DESIGN_v2; …".
- **The probe:** `R/I81/b1_probe_01/PROBE.md`, with RR "I81's B1-0 probe verified; …".
- **The study:** `R/I82/b1_cap_study_01/STUDY.md` and its `ADDENDUM_01.md`.
- **The parent plan:** `R/I61/u8_plan_01/PLAN.md` §2 and §5.2.
- **The milestone record:** `T/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` and `copies/QUALIFICATION.md`, especially §11.
- **The gate set:** RR "T3's gate set and Git rules, consolidated after the handoff was made ephemeral".
- **B6's brief:** `R/BRIEFS/B6_READER_ITEMS.md`.

## Review, in priority order

1. **Coverage.** Every obligation that DESIGN_v2 (T-1 to T-13, §1.3 and §1.4, §2, §3, §6 and the B1 rows of §7), I81's ruling, I82's S3 selection and the measurement obligation, and PLAN §2.1 and §5.2 place on B1 is assigned to a slice, with an acceptance check. List anything missing or assigned twice.
2. **Re-qualification is sufficient** for a change to the D1 call graph and domain. Check it against QUAL §11 and PLAN §2.1:
   - G5, G6 and M's selection rule;
   - the S1 stack witnesses;
   - the challenge peaks;
   - RV87's sweep;
   - Pass A and Pass B;
   - T9 and both-entry;
   - the registered sweep;
   - the full suite before the freeze;
   - DEC-025.

   Check also that its claims about what the milestone already established are true, and that nothing is skipped by appeal to an argument the records don't support.
3. **The slices are workable:**
   - write sets don't overlap across lanes that run at the same time;
   - the dependencies and order hold;
   - the single-writer rule for the corpus holds (B6's 07m first, then 07n);
   - ROOT's ruling points sit where they are needed;
   - the estimates are honest. Compare U8's and T6S's actuals, which are in the records.
4. **The findings are true:**
   - W-C2 can't be pinned before SR-RS;
   - the re-pin cascade on 07m is empty;
   - the milestone is now in the domain, so the six out-of-domain oracles must be re-based;
   - the B2 consequence: a D1-cap combination does not fit beside three cases within 12 GiB.
5. **The decisions:** for each of the 21, AGREE or DISAGREE, with a reason. Check the deciders. Nothing owner-held may be decided; the work graph's T3 section holds the owner-held list, and M ≤ 12 GiB is ROOT's.
6. **PR-B1's packaging:** whether B6 should go first as its own PR (decision 15), and whether PR-B1's gate set is complete.
7. **The measurement method** (§3.6) can produce what the owner needs for the 16 GB floor (measured peak resident memory and run time), on this host, under the lock.

## Host and method

- **Documents and code reading only,** plus read-only Python with VENV against committed files.
- **Not allowed:** cargo, vitest, native or solver jobs, installs, Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Scratch** goes in `WT/scratch/rv107_b1_plan/`. Nothing goes to the system temp directory.

## Output

- **The report:** `R/REVIEW_RV107/b1_plan_01/REVIEW.md` plus SHA256SUMS, with placeholder paths only. It contains:
  - a verdict: PASS, PASS with findings, or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with section, evidence and remedy;
  - a section per item;
  - AGREE or DISAGREE for each decision.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/rv107_b1_plan/records/` and say so.
- **Budget:** 3–4 h.
- **End your turn with:** the verdict, the counts with one line per finding, AGREE or DISAGREE per decision, the report's sha256, and anything ROOT must rule on.
