# B3-S: pricing the exact route at C = 3 (Python only; records only)

TASK (Type 2), dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Cite the records, and assume nothing beyond them.

## Why

B3b brings the 0.3.0 exact route (explicitly empty pressure regions, under `physics-retained-1`) into the retained transaction. Its memory is unpriced: I93's plan priced B2's combinations, but not B3b.
- **Decision 26** (`R/I93/b2b3_plan_01/PLAN.md` §5): price the exact route before B3b's admission. If it does not fit, use route caps or M up to 12 GiB.
- **The target** is S3's shape (C = 3 at D1's caps; also C_eq = 3 with c + z ≤ 3), under the owner's M ≤ 12 GiB, with the 5 % text budget.

## Method

Use I82's pricing chain, with its method in `R/I82/b1_cap_study_01/STUDY.md` and `ADDENDUM_01.md` and the tools in its `_run_records/`, unchanged except for the rebinding below. I65's `u4_g7_06` chain is a reference.

- **Price on a `git archive` snapshot of main** (`2007709549`), in your scratch.
- **Rebind the D1.3 zero rules** to the exact route (PLAN §0 item 9):
  - `append_exact_pressure_results`;
  - `composite.rs`;
  - `pressure_material.rs`;
  - the `build_pressure_case_with_members` edges;
  - the exact route's precommit validator.

  Record each rule rebound, with its multiplicities.
- **Use an exact-route request fixture for the census** (from the physics-source fixtures), at D1's caps.
- **State the tree's change at c = 1** against I82's tree.

## What to give (`STUDY.md`)

- E + R per mode at C = 3 and at C_eq = 3;
- the 5 % M;
- the binding terms;
- the rules rebound, with their multiplicities;
- the comparison with S3's price;
- **decision 26's answer:** whether the exact route fits at M ≤ 12 GiB, and if not, the route caps that would make it fit, priced;
- limits: what is emulated, and what G5 on the real code must replace.

## Rules

- **Python (VENV) and reading only.** Pricing only needs Python and committed files.
- **Not allowed:** cargo, native jobs, installs, and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`. If the chain needs a cargo build, stop and say so; ROOT will run it under the lock.
- **Writes:** absolute paths only. Scratch goes in `WT/scratch/<id>_b3_s/`. Records hold placeholder paths only. No record folder is named `build`.

## Output

- **The record:** `R/<id>/b3_s_01/STUDY.md`, with `_run_records/` (the scripts and their outputs) and SHA256SUMS.
- **Budget:** 3–5 h.
- **End your turn with:**
  - STUDY.md's sha256;
  - the E + R table;
  - the 5 % M;
  - decision 26's answer;
  - anything ROOT must rule on.
