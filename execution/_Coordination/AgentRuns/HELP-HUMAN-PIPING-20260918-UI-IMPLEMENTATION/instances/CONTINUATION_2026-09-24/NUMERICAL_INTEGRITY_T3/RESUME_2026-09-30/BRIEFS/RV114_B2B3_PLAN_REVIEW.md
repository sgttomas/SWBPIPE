# RV114: independent review of the B2/B3 plan (documents only)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this plan.** ROOT rules on the plan only after your review.
- It sizes B2/B3 at about 195–300 h agent and 60–90 h review, roughly three times the parent roadmap's figure.
- It makes FK (the solver kernel), a stop for B1, planned scope for B2.

An error here is expensive.

## The candidate

- **The plan:** `R/I93/b2b3_plan_01/PLAN.md` (sha256 `e1147dbda247c470b2127dbf9413dd10213227216da95495814715e6f8a1238a`), with its `_run_records/` (pricing scripts and outputs).
- **The brief it answered:** `R/BRIEFS/B2B3_PLAN.md` (`174550b0…`).

## Its basis

- **The parent roadmap:** `R/I61/u8_plan_01/PLAN.md` §2.1–§2.2, and decision 8.
- **B0's contract:** `R/I78/b0_contract_01/DESIGN_v2.md` (what B0 left for B2 and B3; decisions 17–19; the reserved names), with RR "B0 selected on DESIGN_v2; …".
- **B1's plan and its S3 study:** `R/I84/b1_plan_01/PLAN_v2.md`; `R/I82/b1_cap_study_01/` with I82's evaluator.
- **The rulings:** RR from "R1: B1's plan ruled …" onward, including:
  - RV78-N1 going to B3's table;
  - the owner's M and target-machine decisions;
  - the B1 notes routed to B2 and B3.
- **The code:** at NUM's maintained tree (main `2007709549`), and B1's in-flight branches (read-only, moving).

## Review, in priority order

1. **The B2 kernel finding is true.**
   - Check against FK's code that `RecordedInvocation::solve_combination` accepts only selected operands.
   - Check that the product certificate refuses combination owners (`product_case_owner`; `SourceBridgeViewIssue::UnsupportedCombination`).
   - Check that B2 therefore needs SC1's prepared-operand API and a combination certificate.
   - Is the plan's B2-K scope right, too small or too large? Is a fresh numerical reviewer the right control?
2. **The memory answer is right.**
   - Re-run I93's pricing with I82's evaluator, independently: the LOW, MID and HIGH constructions, and 3 + 1 and 2 + 1 at D1's caps.
   - Check the claim that a D1-cap combination beside three D1-cap cases does not fit within 12 GiB with the 5 % text budget.
   - Check the proposed cap shape c + z ≤ 3 and its price against S3.
3. **Coverage.** Every obligation B0, I61, B1's routing and the rulings place on B2 or B3 is assigned to a slice with an acceptance check:
   - C3a;
   - the explicit-row rule;
   - RV78-N1's policies;
   - the T6S consistency notes;
   - decisions 17–19;
   - `physics-retained-1`;
   - `legacy_pressure_v1`.

   List anything missing or assigned twice.
4. **The units and order are workable:**
   - B2 as one unit with two stages, and B3 split into B3a and B3b;
   - phase 0 (designs and probes) now, beside B1;
   - lane write sets that don't overlap;
   - the single corpus writer (07o);
   - one qualification and one PR-B2, with a split fallback;
   - honest estimates: compare B1's actuals so far and U8's and T6S's.
5. **The decisions.** For each of the 27, AGREE or DISAGREE, with a reason. Check the deciders: nothing owner-held may be decided.
   - The plan routes decisions 17–19, R-COMB-1's information to the owner, and B4's question with R9.
   - Say what the owner must be told before phase 0, if anything.
6. **Revising the preview table in place** (decision 9). Its hash is pinned in 12 maintained files; check the cascade, and whether this conflicts with B1's freeze or B6's rulings.
7. **What would change the owner's F2a order.** Check I93's list.

## Host and method

- **Documents and code reading only,** plus read-only Python with VENV against committed files, including I82's evaluator.
- **Not allowed:** cargo, vitest, native or solver jobs, installs, Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Writes:** absolute paths only. Scratch goes in `WT/scratch/rv114_b2b3_plan/`. Records hold placeholder paths only. No record folder is named `build`.

## Output

- **The report:** `R/REVIEW_RV114/b2b3_plan_01/REVIEW.md` with `evidence/` and SHA256SUMS. It contains:
  - a verdict: ACCEPT, ACCEPT WITH AMENDMENTS, or REVISE;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - the findings table;
  - a section per item;
  - the decision table.
- **Budget:** 4–6 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - what ROOT must rule on;
  - anything the owner must be told.
