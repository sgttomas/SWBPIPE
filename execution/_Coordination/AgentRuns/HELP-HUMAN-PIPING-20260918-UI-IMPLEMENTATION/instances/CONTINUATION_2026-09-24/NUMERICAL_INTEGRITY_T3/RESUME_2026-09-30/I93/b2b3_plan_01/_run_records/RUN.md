# I93: how the pricing in PLAN.md §3.3 was run

Python 3.13 from VENV; read-only against committed records; outputs written to `WT/scratch/i93_b2b3_plan/` and copied here. No cargo, no Git write, nothing in the system temp directory. Placeholders as in PLAN.md (WT, R, VENV).

**Inputs (unchanged, committed):**
- `R/I82/b1_cap_study_01/_run_records/b1_eval.py` (I82's in-build evaluator) and its `profile_trees/` and `addendum_01/profile_trees/` (sums in PLAN.md §9);
- `R/I72/u8_passb_01/_run_records/runs/u8/law_record.txt` (the registered dev/test build's in-build atoms);
- `R/I65/u4_g7_01/_run_records/pass_a/text_g7/text_budget_W.caps.out.json` (I65's c = 1 TEXT site rows).

**Commands** (the scratch folder also held `b2_bracket_lib.py`, which `b2_mid.py` imports):

```sh
cd WT/scratch/i93_b2b3_plan
PYTHONDONTWRITEBYTECODE=1 VENV/bin/python b2_bracket.py \
  R/I82/b1_cap_study_01/_run_records \
  R/I72/u8_passb_01/_run_records/runs/u8/law_record.txt \
  b2_bracket.out.json
PYTHONDONTWRITEBYTECODE=1 VENV/bin/python b2_mid.py \
  R/I82/b1_cap_study_01/_run_records \
  R/I72/u8_passb_01/_run_records/runs/u8/law_record.txt \
  R/I65/u4_g7_01/_run_records/pass_a/text_g7/text_budget_W.caps.out.json \
  b2_mid.out.json
```

**What each produces:**
- `b2_bracket.out.json`: every tree's in-build E+R per mode; LOW and HIGH for (c, z) ∈ {(1,1), (2,1), (1,2), (3,1), (2,2), (3,3), (2,4), (3,5)} with the 12 GiB budget test; the reduced-cap C_eq = 4 trees' E+R, text budget at 12 GiB and 5 % step; T12 per case.
- `b2_mid.out.json`: the TEXT share (0.651) with its by-file split, and MID for (3,1), (2,1), (1,2), (2,2), (1,1) with the O_base row-atom deltas, the 12 GiB margin, text budget and 5 % step.

**The table.** PLAN.md §3.3's table, its 5 % steps and its heap fractions come from `b2_table.py` over the two outputs:

```sh
PYTHONDONTWRITEBYTECODE=1 VENV/bin/python b2_table.py b2_bracket.out.json b2_mid.out.json > b2_table.out.txt
```

The rule: the smallest 256 MiB multiple M with E+R + 0.05·TAV_W ≤ 0.9 M, with TAV_W from the c tree for LOW, from the c + z tree for HIGH, and c + 0.651·(increment) for MID. The plan quotes the larger of the two modes. The two tier-2 rows at n = m = g ≤ 20 and ≤ 16 with l = 128 are I82's own points (`R/I82/b1_cap_study_01/_run_records/report.json`, `t_c4_k20_l128` and `t_c4_k16_l128`), with the same 5 % rule applied.
