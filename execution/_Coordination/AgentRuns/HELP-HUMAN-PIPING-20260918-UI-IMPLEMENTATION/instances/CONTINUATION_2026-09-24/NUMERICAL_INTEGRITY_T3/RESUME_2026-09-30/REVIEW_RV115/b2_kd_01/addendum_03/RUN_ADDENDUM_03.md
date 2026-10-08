# RV115 ADDENDUM_03: execution record

One standard-library Python script of my own, run once on 2026-10-08 UTC with VENV's Python 3.13.14. It exited with status 0.

```
cd WT/scratch/rv115_b2_kd
TMPDIR=WT/scratch/rv115_b2_kd PYTHONDONTWRITEBYTECODE=1 VENV/bin/python -B rv115_addendum03_checks.py \
  R/I97/b2_c_01/statics/r1/retained_precision_prepared_combination_v1.json \
  R/I97/b2_c_01/statics/retained_precision_prepared_combination_v1.json \
  P/fixtures/results/retained_precision_prepared_ordinary_v1.json > rv115_addendum03_checks.out.json
```

**What it reads and writes:**
- It reads only its three arguments: DEF-C r1, DEF-C v0 and DEF-O.
- It imports nothing from the repository; hypot, sqrt and the canonical form are re-implemented exactly.
- It writes only to stdout.
- The random studies are seeded (`random.Random(115003)`).

The script and its output were copied unchanged into `addendum_03/` (`cmp`, identical).

| Output key | Check |
|---|---|
| `1_gate` | Per 3,000 rows and per row shape: the gate's exact and binary64 sharper predicates on a combination displacement magnitude with a point dual enclosure, at S* = n, 4n, 64n and 8192n, for:<br>• v0's recipe;<br>• r1's with a correctly rounded hypot;<br>• r1's with an adversarial faithful hypot;<br>• one rounding of the exact 3-norm.<br>Also: the coverage bound's check, the largest \|p − ‖x̂‖\| in ulps, and whether the components pass |
| `3_guard_two_faithful_libraries` | The 64ε guard between a correctly rounded and an adversarial faithful nested hypot, subnormals included, as a ratio of the allowance |
| `4_defc_r1_vs_v0` | The changed leaf paths and their r1 texts |
| `5_hash` | H of DEF-C r1, with DEF-O as the control; canonical bytes; `support_magnitude` equal to DEF-O's |
| `6_mm_si_double_rounding_example` | One explicit point-enclosure row where DEF-O's mm→SI projection alone exceeds the sharper allowance (NC-1) |

No cargo, native job, install or Git write. Nothing was written to the system temp directory.
