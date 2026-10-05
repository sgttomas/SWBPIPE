# RV86: independent review of U5, the milestone's reference comparison

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path, and you do not delegate. **You did not write this comparison. Don't rely on its report as your oracle.**

## Why it matters

The milestone requires that the facade-published successor "matches its independent reference". U5 is that evidence. It compares the pinned milestone successors, in both modes, against I50's named oracle, which uses exact rationals and directed bounds. Your review decides whether the evidence establishes that claim, and with what limits.

## The candidate

- **The records:** `R/I61/u5_reference_01/`, at NUM (your dispatch prompt gives the revision): RETURN.md, `_run_records/u5_compare.py`, `run_u5.sh`, `u5_report.json`, `u5_run.log`, `inputs_sha256.txt` and SHA256SUMS.
- **The inputs:**
  - U1's pinned successors: sparse `ac6986b0…` (receipt `efc1a39b…`) and dense `6cd1d249…` (receipt `3e26499f…`). They are written by PP's committed test, and are byte-identical to the stub run of the actual Direct entry in `R/I61/u3_facade_02/`.
  - I50's oracle, `R/I50/first_publishing_component_02/named_oracle.py`.
  - I50's captured log at `WT/scratch/i50_first_publishing/runtime02/pp_debug_final.log`. This is a **local-only dependency**; its hash is in I50's BULK_MANIFEST.
  - The accepted Python reader at NUM, used through `_validate_draft` with eligibility off.
- **The plan's U5 definition:** `R/I61/step4_plan_01/PLAN.md` §2: "every relative-verified and absolute-verified row must agree within its published class", with the prepared route's maxima, overlay and support rows mapped and never reinterpreted.

## Review, in priority order

1. **The oracle reuse is faithful.** `u5_compare.py` reuses the oracle by exact text slices. Confirm that each slice is unmodified oracle text, that the hash assertions bind the right files, and that nothing in the glue changes the oracle's derivation or its tolerances.
2. **The comparison is complete and correctly mapped.**
   - Every published class claim in both successors is compared: 97 per mode, made up of 25 relative, 69 absolute and 3 input-derived.
   - The mapping of the prepared route's maxima patches, overlay headlines and support rows to oracle quantities is correct, with no reinterpretation.
   - Count independently, from the successor bytes, how many rows each class has.
3. **Rerun it yourself.** Run the script unchanged and confirm that the report reproduces byte for byte or value for value. Then add at least three negative controls of your own:
   - perturb one value by one ulp beyond its class;
   - swap two rows' classes;
   - corrupt one bound.
   
   Each must be refused.
4. **The informational finding.** The stop-rule-sharp bound misses on 7 J-dependent rows against the oracle's *represented* readout, though it passes against the source-annulus readout. These are N1's y and z rotations and the five torsional-shear stations. I61 attributes this to the receipt's section terms, which are the prepared route's annulus values and differ from the ordinary A and Z by 2 ulps.
   - Is that attribution correct?
   - Does it affect the class claims, which pass?
   - Is it a defect anywhere, in the producer, the contract or the comparison, or a faithful consequence of the representation?
   - Say plainly whether "matches its independent reference" holds, and with what stated limit.
5. **The extrema intervals.** I61 did not assert that they enclose the truth, because the oracle states no such check. Say whether the milestone claim needs that check, and whether the oracle could supply it cheaply.
6. **Reproducibility.** The I50 log is local-only. Say what a later reviewer needs in order to reproduce U5 if that scratch file is lost, and whether its content could be committed or derived.

## Host and method

- **Records review.** Python only; Git reads only, with `GIT_OPTIONAL_LOCKS=0`. Write under `NUM/R/REVIEW_RV86/u5_reference_01/` and `WT/scratch/rv86_u5_reference_01/`, never the system temp directory.
- **Never:** Cargo, installs, new tooling, or solver, native or DEC-025 jobs.
- **The memory guard** must be running.
- **Other TASKs are working;** don't touch their files. They are I65 (U4 G4), I66 (U6 scoping), RV85 (U3 grant 1b) and possibly I61.

## Output

- **The report:** `NUM/R/REVIEW_RV86/u5_reference_01/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item.
  
  Include a SHA256SUMS. Use placeholder paths only.
- **Time box:** 2 hours.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
