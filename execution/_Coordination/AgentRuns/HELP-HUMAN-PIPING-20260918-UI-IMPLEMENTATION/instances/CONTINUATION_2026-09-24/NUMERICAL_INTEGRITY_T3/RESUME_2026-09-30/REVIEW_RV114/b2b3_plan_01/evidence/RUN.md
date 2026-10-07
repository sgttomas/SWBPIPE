# RV114: how the evidence was produced

Read-only throughout. No cargo, vitest, native or solver job, no install, no Git write; Git reads used `GIT_OPTIONAL_LOCKS=0`. Python 3.13 from VENV with `PYTHONDONTWRITEBYTECODE=1` and `TMPDIR=WT/scratch/rv114_b2b3_plan/tmp`. Scratch was `WT/scratch/rv114_b2b3_plan/` only; nothing was written to the system temp directory. Placeholders as in the dispatch.

**1. I93's pricing, rerun unchanged.** The three scripts were copied from `R/I93/b2b3_plan_01/_run_records/` to `WT/scratch/rv114_b2b3_plan/i93_rerun/` and run there with I93's own command lines (`R/I93/b2b3_plan_01/_run_records/RUN.md`). Each output was compared with `cmp` against I93's committed output: all three identical (`checks.txt` §9).

**2. The independent pricing.** `rv114_price.py` (here) was written fresh. It imports only I82's `b1_eval.py` (sha256 checked by the script, `c404e8db…`) and reads I72's law record (`cbf34c52…`), I82's committed profile trees and I65's c = 1 TEXT rows. It does not import I93's scripts or their form sets; its per-form rule is derived from which forms actually differ between the c and c + 1 trees, and any differing form it does not name stops it.

```sh
cd WT/scratch/rv114_b2b3_plan/indep
PYTHONDONTWRITEBYTECODE=1 VENV/bin/python rv114_price.py \
  R/I82/b1_cap_study_01/_run_records \
  R/I72/u8_passb_01/_run_records/runs/u8/law_record.txt \
  R/I65/u4_g7_01/_run_records/pass_a/text_g7/text_budget_W.caps.out.json \
  rv114_price.out.json
```

It ran once, in seconds. Its LOW and HIGH (h = c) equal I93's to the byte at every (c, z) point I93 reports; it adds HIGH with h = 3 and a file-level TEXT split.

**3. `checks.txt`.** Shell reads (`shasum`, `grep`, `sed`, `git grep`, `git diff --quiet`) and small read-only Python reads of committed JSON, with absolute paths rewritten to placeholders. §10 (MID's sensitivity) and §11 (estimate arithmetic) were computed with stdlib Python from the outputs above and PLAN.md's tables.
