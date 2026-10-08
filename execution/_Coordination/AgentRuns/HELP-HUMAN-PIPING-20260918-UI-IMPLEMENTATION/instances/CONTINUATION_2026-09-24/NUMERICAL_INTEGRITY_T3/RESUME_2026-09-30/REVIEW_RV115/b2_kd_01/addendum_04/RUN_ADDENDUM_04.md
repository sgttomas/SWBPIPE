# RV115 ADDENDUM_04: execution record

One standard-library Python script of my own, run on 2026-10-08 UTC with VENV's Python 3.13.14. The final run exited with status 0.

```
cd WT/scratch/rv115_b2_kd
TMPDIR=WT/scratch/rv115_b2_kd PYTHONDONTWRITEBYTECODE=1 VENV/bin/python -B rv115_addendum04_checks.py \
  R/I97/b2_c_01/_run_records/r2/b2c_checks_r2.py \
  R/I97/b2_c_01/_run_records/r2/exact_norm_vectors.json \
  R/I97/b2_c_01/statics/r1/retained_precision_prepared_combination_v1.json \
  R/I97/b2_c_01/statics/r2/retained_precision_prepared_combination_v1.json \
  P/fixtures/results/retained_precision_prepared_ordinary_v1.json > rv115_addendum04_checks.out.json
```

**What it reads and writes:**
- It reads only its five arguments.
- From I97's script it imports and calls `rn64_norm3` only, as the function under test.
- My oracle, hypot model and pipeline model are my own.
- It writes only to stdout.
- The random studies are seeded (115003, 4 and 77).

**The draft runs.** Two drafts preceded the final run:
- **the first** stopped on the vectors' `"refused"` output encoding;
- **the second** used too small a range for the 2^-1022 trap search, and treated an overflowing estimate as a mismatch rather than deciding it exactly, as SA4-1 (c) requires.

Their outputs were discarded and are not records. The final script and its output were copied unchanged into `addendum_04/` (`cmp`, identical).

| Output key | Check |
|---|---|
| `1_rn64_norm3_vs_my_oracle` | Agreement over each triple family and I97's 411 vectors |
| `2_A5` | The midpoint and the midpoint + 2^-1074: the exact result, a sum rounded to 1,024 bits, nested `math.hypot` |
| `3_availability_point_enclosure_S_eq_n` | v0 against (ii) on the SA3-1 rows |
| `4_guard` | \|p − r\| against my exact faithful nested hypot |
| `5_formation` | The pipeline model's agreement, the 2^-1022 counterexample, and MAX's edges |
| `6_defc` | DEF-C r2 against r1, H, the DEF-O control and canonical bytes |

No cargo, native job, install or Git write. Nothing was written to the system temp directory.
