# RV95: fresh independent complete review of the F2a D1 milestone PR

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path. You do not delegate. **You wrote none of this code. Earlier reviews are inputs to your ledger, not substitutes for your own reading.**

## The candidate

- **The PR branch:** cut by ROOT from current `origin/main` as two commits:
  1. **a source snapshot:** the 136 maintained files under `projects/chirality-piping/` that the T3 integration branch changes, at the U7 head after RV94's repairs and the comment repair;
  2. **an evidence package** at `T3/IMPLEMENTATION/F2A_D1/`: CHANGE_RECORD, SOURCE_EQUALITY, the citation index and the small copied files (U9 decision 5).
- Your dispatch prompt gives the PR number, the head SHA, and the integration head whose maintained source the snapshot must equal.
- **The plan and rulings:**
  - `R/I61/u9_plan_01/PLAN.md`;
  - RR "U9 planned and ruled: the D1 milestone PR", and every T3 ruling from "Step 4 planned: decisions and dispatch" onward;
  - the handoff's Git and evidence rules (`HANDOFF_2026-10-03_TO_NEXT_ROOT.md`);
  - AGENTS.md's merge policy.

## Review, in priority order

1. **Source equality.**
   - Rerun the SOURCE_EQUALITY commands yourself.
   - Every maintained file in the PR must equal the integration head's, except where a recorded resolution differs. The known one is `compatibility.py`, U9 decision 3.
   - No execution-records bulk may be in the PR. The evidence package must be exactly as listed.
2. **The complete diff against main, read in full.**
   - Build a ledger mapping each changed file, and each hunk group, to the review that covered it (RV82, RV85–RV94, and RV89's Pass B reviews), with the reviewed commit.
   - **Read every hunk no review covered, or that changed after its review, yourself.** Absorbing main is the main source of such hunks.
3. **Scope truthfulness.**
   - The PR body, CHANGE_RECORD and the maintained comments say exactly what is public:
     - the D1 milestone through the Direct entry in the registered dev/test build only, with M = 4,026,531,840 B;
     - three-language reader eligibility and the carriers.
   - They also say what stays closed:
     - public activation and product callers;
     - Stale builds;
     - any supported-machine statement of M;
     - U8, wider F2a, S-I, F2b and F3;
     - the owner-held items.
   - No maintained text may claim more. Check the six stale-comment repairs, and that every citation resolves through the index.
4. **Main's interaction.**
   - Check main's changes since the F2a base against the F2a code, above all PR1080's `source_blocks.rs` and PR1078's `compatibility.py`.
   - Confirm that the full Pass B on the PR head (I65, confirmed by RV89) covers PR1080, and that the registered entry's reviewed inputs and PP's `Cargo.lock` are unchanged.
5. **Gate evidence** (U9 decision 12). For each gate, check that its recorded evidence exists, ran on the exact head it claims, and shows the expected result:
   - hosted CI and the full-SHA dispatch;
   - the Mac baseline and DEC-025;
   - GEN-8;
   - T9;
   - both-entry;
   - the src-tauri suite;
   - the pressure and coexistence controls through the Direct entry;
   - the Direct-caller scan;
   - the native witness.
   
   **You do not run DEC-025, native jobs or solvers at scale; ROOT does.**
6. **Your own spot checks,** run in your copy:
   - the milestone through the Direct entry in the registered build: the pinned bytes, in both modes;
   - one Stale build: the ordinary bytes;
   - the three readers' eligibility on the live successor with and without its invocation;
   - a sample of the controls.

## Host and method

- **Your copy:** `git archive` of the PR head into `WT/rv95/`, with targets in `WT/targets/rv95/` (plus a Stale target) and logs in `WT/scratch/rv95_u9_01/`. Delete the copies afterwards.
- **Toolchains:**
  - Cargo: the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time;
  - Python and TypeScript as in the earlier reviews, with an untracked `node_modules` symlink.
- **The memory guard** (PID 5387) must be running.
- **Never:** Git writes, installs, or native, solver-at-scale or DEC-025 jobs. Nothing goes to the system temp directory.

## Output

- **The report:** `NUM/R/REVIEW_RV95/u9_01/REVIEW.md` plus SHA256SUMS, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - the ledger;
  - a section per item.
  
  Use placeholder paths only. After repairs, you confirm them (same-reviewer repair confirmation).
- **Time box:** 5 hours.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
