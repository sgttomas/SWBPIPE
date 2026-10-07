# RV105: independent review of B0's contract design (documents only)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this design.** ROOT selects B0's decisions only after your review, so test each claim against the code and the records yourself.

## The candidate

- **The design:** `R/I78/b0_contract_01/DESIGN.md` (sha256 `25a07a66944922721c9e56fde34afbab98d10bdf4ca5a0888bf7fdede2f98a0c`).
- **The brief it answered:** `R/BRIEFS/B0_CONTRACT_AND_IDENTITIES.md`.
- **Its basis:** the code at NUM `ecb541d63f`. Outside `P/execution` that tree is identical to main `f8ed4f0551`, and NUM's later commits change only records and T6S's desktop, schema and test files. Its notation (PP, RE, FK, PY, RS, TS, C1–C3, DN, D2, PLAN, PROBE, QUAL) is defined at its top.

## Review, in priority order

1. **The findings (§0) are true.** Check each against the code and records, by symbol:
   - **F0-1:**
     - D1 attempts W1 on a `CHECKS_PASSED` case, and all three readers refuse a `selected` case whose `solve_quality` is `checks_passed`.
     - W-C1 (`u8_real_input_fallbacks_append_one_notice`, two-body case B) and QUAL §4's W6 stack witness really are ordinary `CHECKS_PASSED`. Check PROBE §4, the probe log and the tests themselves.
     - With c > 1, one passed case that selects would make precommit refuse the whole successor.
   - **F0-2:** case B is `CHECKS_PASSED` in both modes, so the brief's W-C2 (B unavailable at Ceiling) cannot hold under T-4.
   - **F0-3:**
     - The dense parity row comes from a separate `solve_dense` lane with an absolute 1e-12 pivot guard, and it fails silently.
     - The mode row records the sparse-entry lane.
     - So text A (the briefed biconditional) can refuse a valid successor.
   - **F0-4:** the G8 divergences among RS, PY and TS: scope, code, mode code 3, requested mode.
   - **F0-5:** with one `CaseBatchCall`, a pre-Run call failure hits every case, so D38's shape can never sit beside a selected case. C2 §3 settles the S-2 reading.
   - **F0-6:** `physics-retained-1` and `exact_straight_retained_w1a_v2` were already reserved on 2026-10-03 (RR:6960–6970). Rerun the collision check.
   - **F0-7:** the three single-case code sites (D1.9's `l`, `LOAD_CASES`, G-B's `CaseLoads`).
2. **T-4 (decision 1) is faithful to the accepted design.** At c = 1 it changes what D1 does for a `CHECKS_PASSED` input: no W1 and no notice, where today W1 runs and falls back with one notice. Settle the following:
   - Does DN §4.3, C1:101, C2:164 or D2 §4.9.2 already require the trigger per case, so that D1's behaviour is the deviation?
   - Does T-4 change anything published or public, given D1 is in the registered dev/test build only, with no product caller?
   - Is anything owner-held touched: public meaning, observation framing or the native-app witnesses?
   - Is there a faithful alternative to T-4?
   - Which committed tests or witnesses would change? Check §1.3's audit, and look for others it missed, such as tests or corpus entries whose input is `CHECKS_PASSED` and that expect W1.
3. **The contract text T-1 to T-13 is complete and consistent** with C1, C2, C3, DN §4.4 (coexistence and no-attempt), D2 §4.9 and the schema. Look in particular at:
   - the outcome table;
   - T-11's receipt body and its cumulative snapshots;
   - T-12's notice rule, including the receipt-encoding detail on selected-at-abandonment cases only;
   - G4 (exactly one diagnostic per selected or unavailable case).

   Name any case the text does not decide.
4. **F-1's text B (§3), D38's relaxation (§2) and C3a (§4)** are each implementable in all three readers and the producer, with the corpus cases named. B's keying on `w2` is sound. C3a introduces no hidden solve, no fake readiness and no availability exception.
5. **Caps (§6):** the restatement per invocation is correct, and no M or cap value is selected.
6. **The decisions (§8):** each recommendation follows from the text. The deciders are right: the owner-held list in the work graph's T3 section is authoritative. Nothing owner-held is decided.
7. **Effects (§9):** nothing changes the breadth order or the owner's F2a order, and B1's added work is estimated honestly.

## Host and method

- **Documents and code reading only.** You may run read-only Python scripts against the committed corpus and schema with VENV's Python, for example to enumerate `CHECKS_PASSED` inputs.
- No cargo, vitest, native or solver jobs, no installs, and no Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- Scratch goes in `WT/scratch/rv105_b0_01/`. Nothing goes to the system temp directory.

## Output

- **The report:** `R/REVIEW_RV105/b0_01/REVIEW.md`, with SHA256SUMS (and `evidence/` if you keep any), placeholder paths only. It contains:
  - a verdict: PASS, PASS with findings, or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with section, evidence and remedy;
  - a section per item;
  - for each of the 20 decisions, AGREE or DISAGREE, with a reason.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/rv105_b0_01/records/` and say so.
- **Budget:** 3–5 h.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on or put to the owner.
